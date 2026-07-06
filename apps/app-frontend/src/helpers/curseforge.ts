/**
 * CurseForge API integration helpers.
 *
 * Wraps the `plugin:curseforge|*` Tauri commands exposed by the Rust backend
 * in `apps/app/src/api/curseforge.rs`. CurseForge search results are converted
 * to a Modrinth V3-compatible shape on the backend, so the rest of the UI can
 * render them like normal Modrinth results.
 */

import { invoke } from '@tauri-apps/api/core'

import type { InstallJobSnapshot } from './install'

/** A single CurseForge project (mod/modpack/resourcepack/shader/world/datapack). */
export interface CfMod {
	id: number
	game_id: number
	name: string
	slug: string
	summary: string
	download_count: number
	primary_category_id: number
	class_id: number | null
	categories: CfCategory[]
	authors: CfAuthor[]
	logo: CfAsset | null
	screenshots: CfAsset[]
	links: CfLinks
	date_created: string
	date_modified: string
	date_released: string
	allow_mod_distribution: boolean | null
	game_popularity_rank: number | null
	is_available: boolean | null
	latest_files: CfFile[]
	latest_files_indexes: CfFileIndex[]
}

export interface CfCategory {
	id: number
	name: string
	slug: string
	class_id: number | null
	parent_category_id: number | null
	icon_url: string | null
}

export interface CfAuthor {
	id: number
	name: string
	url: string | null
}

export interface CfAsset {
	id: number
	url: string | null
	thumbnail_url: string | null
}

export interface CfLinks {
	website_url: string | null
	wiki_url: string | null
	issues_url: string | null
	source_url: string | null
}

export interface CfFile {
	id: number
	mod_id: number
	is_available: boolean | null
	display_name: string
	file_name: string
	release_type: number // 1=Release, 2=Beta, 3=Alpha
	file_status: number
	file_date: string
	file_length: number
	download_url: string | null
	game_versions: string[]
	sortable_game_versions: CfSortableGameVersion[]
	dependencies: CfFileDependency[]
	alternate_file_id: number
	is_server_pack: boolean | null
	file_fingerprint: number
}

export interface CfFileIndex {
	game_version: string
	file_id: number | null
	filename: string | null
	release_type: number | null
	game_version_type_id: number | null
	mod_loader: number | null
}

export interface CfSortableGameVersion {
	game_version_name: string
	game_version_padded: string
	game_version: string
	game_version_release_date: string | null
	game_version_type_id: number | null
}

export interface CfFileDependency {
	mod_id: number
	file_id: number | null
	relation_type: number
}

/**
 * Modrinth V3-compatible search result hit returned by `cf_search`.
 *
 * CurseForge-specific extras are prefixed with `_cf_` so frontend code can
 * detect the source when needed while reusing all the regular V3 fields.
 */
export interface CfSearchHit {
	project_id: string
	version_id: string
	project_types: string[]
	slug: string
	author: string
	author_id: string
	organization: string | null
	organization_id: string | null
	name: string
	summary: string
	categories: string[]
	display_categories: string[]
	downloads: number
	follows: number
	icon_url: string | null
	date_created: string
	date_modified: string
	license: string
	gallery: string[]
	featured_gallery: string | null
	color: number | null
	loaders: string[]
	versions: string[]
	// CurseForge-specific extras
	_cf_mod_id: number
	_cf_class_id: number
	_cf_page_url: string | null
	_cf_download_count: number
}

export interface CfSearchResults {
	search: string
	result: {
		hits: CfSearchHit[]
		offset: number
		limit: number
		total_hits: number
	}
}

export interface CfSearchParams {
	/** "mod" | "modpack" | "resourcepack" | "shader" | "world" | "datapack" */
	projectType: string
	searchFilter?: string | null
	gameVersion?: string | null
	/** "forge" | "fabric" | "quilt" | "neoforge" | "liteloader" */
	loader?: string | null
	/** "relevance" | "downloads" | "follows" | "newest" | "updated" */
	sort?: string | null
	/** 0-indexed page */
	page?: number | null
	pageSize?: number | null
}

/** Search CurseForge; returns Modrinth V3-compatible results. */
export async function cf_search(params: CfSearchParams): Promise<CfSearchResults> {
	return await invoke<CfSearchResults>('plugin:curseforge|cf_search', { params })
}

/** Get a single CurseForge project by mod ID. */
export async function cf_get_mod(modId: number): Promise<CfMod> {
	return await invoke<CfMod>('plugin:curseforge|cf_get_mod', { modId })
}

/** Get files for a CurseForge project, optionally filtered by game version and loader. */
export async function cf_get_mod_files(
	modId: number,
	gameVersion?: string | null,
	loader?: string | null,
): Promise<CfFile[]> {
	return await invoke<CfFile[]>('plugin:curseforge|cf_get_mod_files', {
		modId,
		gameVersion: gameVersion ?? null,
		loader: loader ?? null,
	})
}

/** Get a single CurseForge file by mod ID and file ID. */
export async function cf_get_file(modId: number, fileId: number): Promise<CfFile> {
	return await invoke<CfFile>('plugin:curseforge|cf_get_file', { modId, fileId })
}

/** Get the direct download URL for a CurseForge file. Returns null if unavailable. */
export async function cf_get_file_download_url(
	modId: number,
	fileId: number,
): Promise<string | null> {
	return await invoke<string | null>('plugin:curseforge|cf_get_file_download_url', {
		modId,
		fileId,
	})
}

/** Get CurseForge categories. If `projectType` is null, returns all categories for Minecraft. */
export async function cf_get_categories(
	projectType?: string | null,
): Promise<CfCategory[]> {
	return await invoke<CfCategory[]>('plugin:curseforge|cf_get_categories', {
		projectType: projectType ?? null,
	})
}

/**
 * Install a CurseForge file to an existing instance via the InstallJob system.
 * The download popup shows progress like other installs.
 *
 * @param contentType "mod" | "resourcepack" | "shader" | "datapack" | "world" | "modpack"
 */
export async function cf_install_file(
	instanceId: string,
	modId: number,
	fileId: number,
	fileName: string,
	downloadUrl: string | null,
	contentType: string,
	title: string,
	iconUrl: string | null,
): Promise<InstallJobSnapshot> {
	return await invoke<InstallJobSnapshot>('plugin:curseforge|cf_install_file', {
		instanceId,
		modId,
		fileId,
		fileName,
		downloadUrl,
		contentType,
		title,
		iconUrl,
	})
}
