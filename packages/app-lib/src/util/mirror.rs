//! Mirror source URL rewriting for downloads
//!
//! Rewrites official Mojang/Modrinth URLs to mirror URLs (BMCLAPI / MCIMirror)
//! to improve download speeds in regions with poor connectivity to official servers.
//!
//! Three strategies per source:
//! - `Mirror`: mirror URL first, fallback to official
//! - `Auto` (default): official URL first, fallback to mirror
//! - `Official`: official URL only, no fallback

use std::sync::atomic::{AtomicI32, Ordering};

use serde::{Deserialize, Serialize};

/// Download source strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "i32", from = "i32")]
pub enum DownloadSource {
    /// Prefer mirror source, fallback to official
    Mirror = 0,
    /// Auto - official first, fallback to mirror (default)
    Auto = 1,
    /// Official source only
    Official = 2,
}

impl DownloadSource {
    pub fn from_i32(val: i32) -> Self {
        match val {
            0 => Self::Mirror,
            1 => Self::Auto,
            2 => Self::Official,
            _ => Self::Auto,
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl Default for DownloadSource {
    fn default() -> Self {
        Self::Auto
    }
}

impl From<i32> for DownloadSource {
    fn from(val: i32) -> Self {
        Self::from_i32(val)
    }
}

impl From<DownloadSource> for i32 {
    fn from(val: DownloadSource) -> i32 {
        val.as_i32()
    }
}

/// Fetch strategy for a specific URL, combining both sources.
/// Determines which URL to try first, and what to fall back to.
#[derive(Debug, Clone)]
pub enum FetchStrategy {
    /// Use official URL only, no mirror fallback
    OfficialOnly,
    /// Try mirror URL first, fallback to official on failure
    MirrorFirst { mirror_url: String },
    /// Try official URL first, fallback to mirror on failure
    OfficialFirstWithFallback { mirror_url: String },
}

// Global cache for mirror settings (avoid DB query on every fetch)
static GAME_FILE_SOURCE: AtomicI32 = AtomicI32::new(1); // Default: Auto
static COMMUNITY_SOURCE: AtomicI32 = AtomicI32::new(1); // Default: Auto

/// Update global mirror settings cache.
/// Called from Settings::get / Settings::update to keep the cache in sync.
pub fn update_mirror_settings(game_source: i32, community_source: i32) {
    GAME_FILE_SOURCE.store(game_source, Ordering::Relaxed);
    COMMUNITY_SOURCE.store(community_source, Ordering::Relaxed);
}

/// Get current game file source setting
pub fn get_game_file_source() -> DownloadSource {
    DownloadSource::from_i32(GAME_FILE_SOURCE.load(Ordering::Relaxed))
}

/// Get current community source setting
pub fn get_community_source() -> DownloadSource {
    DownloadSource::from_i32(COMMUNITY_SOURCE.load(Ordering::Relaxed))
}

/// Determine the fetch strategy for a given URL based on current mirror settings.
///
/// Returns the appropriate strategy considering which source category the URL
/// belongs to (game files vs community resources) and the user's preference.
pub fn get_fetch_strategy(url: &str) -> FetchStrategy {
    // Check game file URLs (Mojang/BMCLAPI)
    if let Some(mirror_url) = rewrite_bmclapi(url) {
        return match get_game_file_source() {
            DownloadSource::Mirror => FetchStrategy::MirrorFirst { mirror_url },
            DownloadSource::Auto => {
                FetchStrategy::OfficialFirstWithFallback { mirror_url }
            }
            DownloadSource::Official => FetchStrategy::OfficialOnly,
        };
    }

    // Check community URLs (Modrinth CDN / MCIMirror)
    if let Some(mirror_url) = rewrite_mcimirror(url) {
        return match get_community_source() {
            DownloadSource::Mirror => FetchStrategy::MirrorFirst { mirror_url },
            DownloadSource::Auto => {
                FetchStrategy::OfficialFirstWithFallback { mirror_url }
            }
            DownloadSource::Official => FetchStrategy::OfficialOnly,
        };
    }

    // No mirror pattern matched - always use official
    FetchStrategy::OfficialOnly
}

/// Rewrite Mojang official URLs to BMCLAPI mirror.
/// Returns Some(rewritten_url) if the URL matched, None otherwise.
fn rewrite_bmclapi(url: &str) -> Option<String> {
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Mojang version manifest / meta
        ("https://piston-meta.mojang.com/", "https://bmclapi2.bangbang93.com/"),
        // Mojang client JAR / data
        ("https://piston-data.mojang.com/", "https://bmclapi2.bangbang93.com/"),
        // Mojang launcher
        ("https://launcher.mojang.com/", "https://bmclapi2.bangbang93.com/"),
        // Mojang launcher meta
        ("https://launchermeta.mojang.com/", "https://bmclapi2.bangbang93.com/"),
        // Minecraft assets
        ("https://resources.download.minecraft.net/", "https://bmclapi2.bangbang93.com/assets/"),
        // Minecraft libraries
        ("https://libraries.minecraft.net/", "https://bmclapi2.bangbang93.com/maven/"),
    ];

    for (from, to) in REPLACEMENTS {
        if url.starts_with(from) {
            return Some(url.replacen(from, to, 1));
        }
    }
    None
}

/// Rewrite Modrinth CDN URLs to MCIMirror.
/// Returns Some(rewritten_url) if the URL matched, None otherwise.
///
/// Note: Only CDN download URLs are mirrored. API URLs are NOT mirrored
/// because the mirror API may return incomplete/incompatible data structures
/// (e.g. missing `raw_url` / `id` fields), causing deserialization errors.
/// Content browsing/discovery always uses the official Modrinth API.
fn rewrite_mcimirror(url: &str) -> Option<String> {
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Modrinth CDN (mod file downloads only)
        ("https://cdn.modrinth.com/", "https://mod.mcimirror.top/"),
    ];

    for (from, to) in REPLACEMENTS {
        if url.starts_with(from) {
            return Some(url.replacen(from, to, 1));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_bmclapi() {
        assert_eq!(
            rewrite_bmclapi("https://piston-data.mojang.com/v1/objects/abc123/client.jar"),
            Some("https://bmclapi2.bangbang93.com/v1/objects/abc123/client.jar".to_string())
        );
        assert_eq!(
            rewrite_bmclapi("https://resources.download.minecraft.net/aa/bb/aabb.file"),
            Some("https://bmclapi2.bangbang93.com/assets/aa/bb/aabb.file".to_string())
        );
        assert_eq!(
            rewrite_bmclapi("https://libraries.minecraft.net/net/java/jinput.jar"),
            Some("https://bmclapi2.bangbang93.com/maven/net/java/jinput.jar".to_string())
        );
        assert_eq!(rewrite_bmclapi("https://example.com/file"), None);
    }

    #[test]
    fn test_rewrite_mcimirror() {
        assert_eq!(
            rewrite_mcimirror("https://cdn.modrinth.com/data/abc123/versions/v1/mod.jar"),
            Some("https://mod.mcimirror.top/data/abc123/versions/v1/mod.jar".to_string())
        );
        // API URLs should NOT be rewritten
        assert_eq!(rewrite_mcimirror("https://api.modrinth.com/v2/project/sodium"), None);
    }

    #[test]
    fn test_fetch_strategy_auto_official_first() {
        // With Auto (default), game file URLs should be official-first with mirror fallback
        update_mirror_settings(1, 1); // Auto, Auto
        match get_fetch_strategy("https://piston-data.mojang.com/v1/objects/abc/client.jar") {
            FetchStrategy::OfficialFirstWithFallback { mirror_url } => {
                assert!(mirror_url.contains("bmclapi2.bangbang93.com"));
            }
            other => panic!("Expected OfficialFirstWithFallback, got {other:?}"),
        }
        match get_fetch_strategy("https://cdn.modrinth.com/data/abc/mod.jar") {
            FetchStrategy::OfficialFirstWithFallback { mirror_url } => {
                assert!(mirror_url.contains("mod.mcimirror.top"));
            }
            other => panic!("Expected OfficialFirstWithFallback, got {other:?}"),
        }
    }

    #[test]
    fn test_fetch_strategy_mirror_first() {
        update_mirror_settings(0, 0); // Mirror, Mirror
        match get_fetch_strategy("https://piston-data.mojang.com/v1/objects/abc/client.jar") {
            FetchStrategy::MirrorFirst { mirror_url } => {
                assert!(mirror_url.contains("bmclapi2.bangbang93.com"));
            }
            other => panic!("Expected MirrorFirst, got {other:?}"),
        }
    }

    #[test]
    fn test_fetch_strategy_official_only() {
        update_mirror_settings(2, 2); // Official, Official
        match get_fetch_strategy("https://piston-data.mojang.com/v1/objects/abc/client.jar") {
            FetchStrategy::OfficialOnly => {}
            other => panic!("Expected OfficialOnly, got {other:?}"),
        }
    }

    #[test]
    fn test_no_mirror_for_non_matching_urls() {
        update_mirror_settings(0, 0); // Even with Mirror mode
        match get_fetch_strategy("https://api.modrinth.com/v2/project/sodium") {
            FetchStrategy::OfficialOnly => {}
            other => panic!("API URLs should always be OfficialOnly, got {other:?}"),
        }
        match get_fetch_strategy("https://example.com/file") {
            FetchStrategy::OfficialOnly => {}
            other => panic!("Non-matching URLs should be OfficialOnly, got {other:?}"),
        }
    }
}
