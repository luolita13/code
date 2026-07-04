//! Yggdrasil (authlib-injector) third-party authentication.
//!
//! Implements the legacy Yggdrasil authentication protocol used by authlib-injector
//! compatible servers (LittleSkin, custom servers). The protocol is based on the
//! original Mojang authentication API and consists of:
//!
//! - `POST {api_root}/authserver/authenticate` — login with username/password
//! - `POST {api_root}/authserver/refresh`      — refresh access token
//! - `POST {api_root}/authserver/validate`     — check if token is still valid
//! - `GET  {api_root}/sessionserver/session/minecraft/profile/{uuid}` — fetch profile
//!
//! The `api_root` is discovered via the `X-Authlib-Injector-Api-Location` header
//! returned by the server root, following the authlib-injector specification.

use crate::state::{Credentials, LoginType, MinecraftProfile};
use crate::util::fetch::INSECURE_REQWEST_CLIENT;
use base64::Engine;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use url::Url;
use uuid::Uuid;

const YGGDRASIL_USER_AGENT: &str =
    "Modrinth App (support@modrinth.com; https://modrinth.com/app)";

#[derive(thiserror::Error, Debug)]
pub enum YggdrasilError {
    #[error("Invalid server URL: {0}")]
    InvalidUrl(String),
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Authentication failed ({status}): {body}")]
    AuthenticationFailed { status: u16, body: String },
    #[error("No profile returned by server")]
    NoProfile,
    #[error("Profile response could not be parsed: {0}")]
    ProfileParse(#[from] serde_json::Error),
}

#[derive(Serialize)]
struct AuthenticateRequest<'a> {
    agent: AuthenticateAgent,
    username: &'a str,
    password: &'a str,
    request_user: bool,
}

#[derive(Serialize)]
struct AuthenticateAgent {
    name: &'static str,
    version: u32,
}

#[derive(Deserialize, Debug)]
struct AuthenticateResponse {
    access_token: String,
    client_token: String,
    selected_profile: Option<YggdrasilProfile>,
    available_profiles: Option<Vec<YggdrasilProfile>>,
}

#[derive(Serialize)]
struct RefreshRequest<'a> {
    access_token: &'a str,
    client_token: &'a str,
    request_user: bool,
}

#[derive(Deserialize, Debug)]
struct RefreshResponse {
    access_token: String,
    client_token: String,
    selected_profile: Option<YggdrasilProfile>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct YggdrasilProfile {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct YggdrasilProfileResponse {
    id: String,
    name: String,
    #[serde(default)]
    properties: Vec<YggdrasilProfileProperty>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct YggdrasilProfileProperty {
    name: String,
    value: String,
}

/// Normalizes the user-provided server URL into an `api_root` string.
///
/// - Trims trailing slashes
/// - Strips a trailing `/authserver` if present (some users paste the authserver URL)
/// - If the URL points to a path ending with `/api/yggdrasil`, that is treated as the api_root
fn normalize_api_root(server_url: &str) -> crate::Result<String> {
    let mut url = server_url.trim().trim_end_matches('/').to_string();

    // Strip trailing /authserver if user pasted the authserver endpoint directly
    if url.ends_with("/authserver") {
        url = url.trim_end_matches("/authserver").to_string();
    }

    if url.is_empty() {
        return Err(crate::ErrorKind::OtherError(
            "Empty Yggdrasil server URL".to_string(),
        )
        .into());
    }

    Ok(url)
}

/// Resolves the api_root, following the `X-Authlib-Injector-Api-Location` header
/// if the server returns one.
///
/// Following authlib-injector conventions:
/// 1. HEAD the user-provided URL
/// 2. If `X-Authlib-Injector-Api-Location` header is present, resolve it as a
///    relative URL against the request URL
/// 3. Otherwise, use the user-provided URL as-is
pub async fn resolve_api_root(server_url: &str) -> crate::Result<String> {
    let base = normalize_api_root(server_url)?;

    let response = INSECURE_REQWEST_CLIENT
        .head(&base)
        .header("User-Agent", YGGDRASIL_USER_AGENT)
        .send()
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to reach Yggdrasil server {base}: {e}"
            ))
            .as_error()
        })?;

    if let Some(location) = response
        .headers()
        .get("X-Authlib-Injector-Api-Location")
        .and_then(|v| v.to_str().ok())
    {
        // Resolve relative location against the request URL
        let parsed = Url::parse(&base)
            .map_err(|e| {
                crate::ErrorKind::OtherError(format!(
                    "Invalid server URL {base}: {e}"
                ))
                .as_error()
            })?;

        let resolved = parsed.join(location).map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to resolve X-Authlib-Injector-Api-Location '{location}': {e}"
            ))
            .as_error()
        })?;

        Ok(resolved.to_string())
    } else {
        Ok(base)
    }
}

/// Performs a full Yggdrasil login flow.
///
/// The resulting `Credentials` stores:
/// - `access_token`  → Yggdrasil access token
/// - `refresh_token` → Yggdrasil client token (used to refresh the access token)
/// - `login_type`    → `Yggdrasil`
/// - `server_url`    → normalized api_root
#[tracing::instrument(skip(password, exec))]
pub async fn login_yggdrasil(
    server_url: &str,
    username: &str,
    password: &str,
    exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite> + Copy,
) -> crate::Result<Credentials> {
    let api_root = resolve_api_root(server_url).await?;

    let body = AuthenticateRequest {
        agent: AuthenticateAgent {
            name: "Minecraft",
            version: 1,
        },
        username,
        password,
        request_user: true,
    };

    let response = INSECURE_REQWEST_CLIENT
        .post(format!("{api_root}/authserver/authenticate"))
        .header("User-Agent", YGGDRASIL_USER_AGENT)
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(YggdrasilError::Request)?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(YggdrasilError::AuthenticationFailed {
            status: status.as_u16(),
            body,
        }
        .into());
    }

    let auth: AuthenticateResponse = response
        .json()
        .await
        .map_err(YggdrasilError::Request)?;

    let profile = auth
        .selected_profile
        .or_else(|| {
            auth.available_profiles
                .as_ref()
                .and_then(|p| p.first().cloned())
        })
        .ok_or_else(|| {
            crate::ErrorKind::OtherError(
                "Yggdrasil server returned no player profile".to_string(),
            )
            .as_error()
        })?;

    let uuid = Uuid::parse_str(&profile.id).unwrap_or_else(|_| Uuid::nil());
    let username = profile.name.clone();

    let credentials = Credentials {
        offline_profile: MinecraftProfile {
            id: uuid,
            name: username,
            ..MinecraftProfile::default()
        },
        access_token: auth.access_token,
        // Store client_token in the refresh_token slot to avoid schema changes.
        refresh_token: auth.client_token,
        // Yggdrasil tokens typically expire after 1 day; use a conservative 1 hour margin.
        expires: Utc::now() + Duration::hours(1),
        active: true,
        login_type: LoginType::Yggdrasil,
        server_url: Some(api_root),
    };

    credentials.upsert(exec).await?;

    Ok(credentials)
}

/// Refreshes Yggdrasil credentials by calling the `/authserver/refresh` endpoint.
///
/// Called from `Credentials::refresh` when `login_type == Yggdrasil`.
pub async fn refresh_credentials(
    creds: &mut Credentials,
    _exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite> + Copy,
) -> crate::Result<()> {
    // Offline accounts never expire; if expires is in the future, do nothing.
    if creds.expires > Utc::now() {
        return Ok(());
    }

    let api_root = creds.server_url.as_ref().ok_or_else(|| {
        crate::ErrorKind::OtherError(
            "Yggdrasil credentials missing server_url".to_string(),
        )
        .as_error()
    })?;

    let body = RefreshRequest {
        access_token: &creds.access_token,
        client_token: &creds.refresh_token,
        request_user: true,
    };

    let response = INSECURE_REQWEST_CLIENT
        .post(format!("{api_root}/authserver/refresh"))
        .header("User-Agent", YGGDRASIL_USER_AGENT)
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(YggdrasilError::Request)?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(YggdrasilError::AuthenticationFailed {
            status: status.as_u16(),
            body,
        }
        .into());
    }

    let refreshed: RefreshResponse = response
        .json()
        .await
        .map_err(YggdrasilError::Request)?;

    creds.access_token = refreshed.access_token;
    if !refreshed.client_token.is_empty() {
        creds.refresh_token = refreshed.client_token;
    }

    if let Some(profile) = refreshed.selected_profile {
        creds.offline_profile.id =
            Uuid::parse_str(&profile.id).unwrap_or(creds.offline_profile.id);
        creds.offline_profile.name = profile.name;
    }

    creds.expires = Utc::now() + Duration::hours(1);

    Ok(())
}

/// Validates whether the Yggdrasil access token is still valid by calling
/// `/authserver/validate`. Returns `true` if the token is still usable.
#[allow(dead_code)]
pub async fn validate(
    api_root: &str,
    access_token: &str,
    client_token: &str,
) -> crate::Result<bool> {
    let body = serde_json::json!({
        "access_token": access_token,
        "client_token": client_token,
    });

    let response = INSECURE_REQWEST_CLIENT
        .post(format!("{api_root}/authserver/validate"))
        .header("User-Agent", YGGDRASIL_USER_AGENT)
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(YggdrasilError::Request)?;

    Ok(response.status().is_success())
}

/// Fetches a Minecraft profile from a Yggdrasil session server.
///
/// Useful for retrieving skins and capes contributed by the third-party server.
#[allow(dead_code)]
pub async fn fetch_profile(
    api_root: &str,
    uuid: Uuid,
) -> crate::Result<Option<MinecraftProfile>> {
    let url = format!(
        "{api_root}/sessionserver/session/minecraft/profile/{}",
        uuid.simple()
    );

    let response = INSECURE_REQWEST_CLIENT
        .get(&url)
        .header("User-Agent", YGGDRASIL_USER_AGENT)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(YggdrasilError::Request)?;

    if response.status() == reqwest::StatusCode::NO_CONTENT
        || response.status() == reqwest::StatusCode::NOT_FOUND
    {
        return Ok(None);
    }

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(YggdrasilError::AuthenticationFailed {
            status: status.as_u16(),
            body,
        }
        .into());
    }

    let parsed: YggdrasilProfileResponse = response
        .json()
        .await
        .map_err(YggdrasilError::Request)?;

    let id = Uuid::parse_str(&parsed.id).unwrap_or(uuid);

    Ok(Some(MinecraftProfile {
        id,
        name: parsed.name,
        skins: Vec::new(),
        capes: Vec::new(),
        fetch_time: None,
    }))
}

/// Returns the `authlib-injector.jar` prefetch payload (Base64-encoded server
/// metadata) to be passed via `-Dauthlibinjector.yggdrasil.prefetched`.
///
/// This avoids the runtime network probe of the Yggdrasil server during Minecraft
/// launch, improving startup reliability.
pub async fn build_prefetch_payload(
    api_root: &str,
) -> crate::Result<Cow<'static, str>> {
    let response = INSECURE_REQWEST_CLIENT
        .get(api_root)
        .header("User-Agent", YGGDRASIL_USER_AGENT)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(YggdrasilError::Request)?;

    let body = response.text().await.map_err(YggdrasilError::Request)?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(body);
    Ok(Cow::Owned(encoded))
}

/// Canonical release URL for the authlib-injector artifact.
///
/// Points to the latest released `authlib-injector.jar` on GitHub. The artifact
/// is ~500KB and stable across releases, so we cache it locally and only
/// re-download when the file is missing.
const AUTHLIB_INJECTOR_URL: &str =
    "https://github.com/to2mbn/authlib-injector/releases/latest/download/authlib-injector.jar";

/// Ensures the `authlib-injector.jar` is present in the Theseus caches
/// directory, downloading it on first use. Returns the absolute path to the
/// cached jar file.
///
/// This is invoked by the launcher when starting Minecraft with a Yggdrasil
/// account, so the jar can be passed via `-javaagent:`.
pub async fn ensure_authlib_injector(
    caches_dir: &std::path::Path,
) -> crate::Result<std::path::PathBuf> {
    let jar_path = caches_dir.join("authlib-injector.jar");

    if jar_path.exists() {
        return Ok(jar_path);
    }

    if let Some(parent) = jar_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| {
                crate::ErrorKind::OtherError(format!(
                    "Failed to create caches dir for authlib-injector: {e}"
                ))
                .as_error()
            })?;
    }

    let response = INSECURE_REQWEST_CLIENT
        .get(AUTHLIB_INJECTOR_URL)
        .header("User-Agent", YGGDRASIL_USER_AGENT)
        .send()
        .await
        .map_err(YggdrasilError::Request)?;

    if !response.status().is_success() {
        return Err(crate::ErrorKind::OtherError(format!(
            "Failed to download authlib-injector.jar (HTTP {})",
            response.status()
        ))
        .into());
    }

    let bytes = response
        .bytes()
        .await
        .map_err(YggdrasilError::Request)?;

    if bytes.len() < 1024 {
        return Err(crate::ErrorKind::OtherError(format!(
            "Downloaded authlib-injector.jar is suspiciously small ({} bytes) — aborting",
            bytes.len()
        ))
        .into());
    }

    tokio::fs::write(&jar_path, &bytes).await.map_err(|e| {
        crate::ErrorKind::OtherError(format!(
            "Failed to write authlib-injector.jar to {}: {e}",
            jar_path.display()
        ))
        .as_error()
    })?;

    tracing::info!(
        "Downloaded authlib-injector.jar ({} bytes) to {}",
        bytes.len(),
        jar_path.display()
    );

    Ok(jar_path)
}
