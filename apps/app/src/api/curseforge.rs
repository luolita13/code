//! Tauri plugin bindings for CurseForge API integration.
//!
//! Exposes curseforge search, project lookup, file listing, and a
//! InstallJob-driven file installer to the frontend.

use crate::api::Result;
use serde::Deserialize;
use theseus::curseforge::{
    CfClassId, CfFile, CfMod, CfModLoaderType, CfSortField, CfSortOrder,
    CfSearchParams,
};
use theseus::install::InstallJobSnapshot;
use theseus::prelude::*;

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("curseforge")
        .invoke_handler(tauri::generate_handler![
            cf_search,
            cf_get_mod,
            cf_get_mod_files,
            cf_get_file,
            cf_get_file_download_url,
            cf_get_categories,
            cf_install_file,
        ])
        .build()
}

/// Search parameters mirroring `CfSearchParams`, with camelCase fields
/// so the frontend can pass plain JS objects.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CfSearchParamsRequest {
    /// Project type string ("mod"|"modpack"|"resourcepack"|"shader"|"world"|"datapack")
    pub project_type: String,
    #[serde(default)]
    pub search_filter: Option<String>,
    #[serde(default)]
    pub game_version: Option<String>,
    /// Loader name ("forge"|"fabric"|"quilt"|"neoforge"|"liteloader") or null
    #[serde(default)]
    pub loader: Option<String>,
    /// Modrinth sort name ("relevance"|"downloads"|"follows"|"newest"|"updated")
    #[serde(default)]
    pub sort: Option<String>,
    /// 0-indexed page
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub page_size: Option<u32>,
}

/// Search CurseForge and return Modrinth V3-compatible results.
#[tauri::command]
pub async fn cf_search(params: CfSearchParamsRequest) -> Result<SearchResultsV3> {
    let state = State::get().await?;
    let class_id = CfClassId::from_project_type(&params.project_type)
        .ok_or_else(|| {
            theseus::Error::from(theseus::ErrorKind::InputError(format!(
                "Unknown project_type: {}",
                params.project_type
            )))
        })?;

    let mod_loader_type = params
        .loader
        .as_deref()
        .and_then(CfModLoaderType::from_loader_name);

    let sort_field = params
        .sort
        .as_deref()
        .map(CfSortField::from_modrinth_sort);

    let search_params = CfSearchParams {
        class_id,
        search_filter: params.search_filter,
        game_version: params.game_version,
        mod_loader_type,
        sort_field,
        sort_order: Some(CfSortOrder::Descending),
        page: params.page,
        page_size: params.page_size,
    };

    Ok(theseus::curseforge::search(search_params, &state).await?)
}

/// Get a single CurseForge mod by ID.
#[tauri::command]
pub async fn cf_get_mod(mod_id: i64) -> Result<CfMod> {
    let state = State::get().await?;
    Ok(theseus::curseforge::get_mod(mod_id, &state).await?)
}

/// Get files for a CurseForge mod, optionally filtered.
#[tauri::command]
pub async fn cf_get_mod_files(
    mod_id: i64,
    game_version: Option<String>,
    loader: Option<String>,
) -> Result<Vec<CfFile>> {
    let state = State::get().await?;
    let loader_type = loader
        .as_deref()
        .and_then(CfModLoaderType::from_loader_name);
    Ok(theseus::curseforge::get_mod_files(
        mod_id,
        game_version.as_deref(),
        loader_type,
        &state,
    )
    .await?)
}

/// Get a single CurseForge file by mod_id and file_id.
#[tauri::command]
pub async fn cf_get_file(mod_id: i64, file_id: i64) -> Result<CfFile> {
    let state = State::get().await?;
    Ok(theseus::curseforge::get_file(mod_id, file_id, &state).await?)
}

/// Get the direct download URL for a CurseForge file.
/// Returns null if the file is not available for third-party download.
#[tauri::command]
pub async fn cf_get_file_download_url(
    mod_id: i64,
    file_id: i64,
) -> Result<Option<String>> {
    let state = State::get().await?;
    Ok(theseus::curseforge::get_file_download_url(mod_id, file_id, &state)
        .await?)
}

/// Get CurseForge categories for a given class.
/// If project_type is null, returns all categories for Minecraft.
#[tauri::command]
pub async fn cf_get_categories(
    project_type: Option<String>,
) -> Result<Vec<theseus::curseforge::CfCategory>> {
    let state = State::get().await?;
    let class_id = project_type
        .as_deref()
        .and_then(CfClassId::from_project_type);
    Ok(theseus::curseforge::get_categories(class_id, &state).await?)
}

/// Install a CurseForge file to an existing instance via the InstallJob
/// system. Displays progress in the download popup like other installs.
#[tauri::command]
pub async fn cf_install_file(
    instance_id: String,
    mod_id: i64,
    file_id: i64,
    file_name: String,
    download_url: Option<String>,
    content_type: String,
    title: String,
    icon_url: Option<String>,
) -> Result<InstallJobSnapshot> {
    Ok(theseus::install::install_curseforge_file(
        instance_id,
        mod_id,
        file_id,
        file_name,
        download_url,
        content_type,
        title,
        icon_url,
    )
    .await?)
}
