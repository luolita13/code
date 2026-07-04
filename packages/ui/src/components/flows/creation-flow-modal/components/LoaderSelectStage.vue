<template>
	<div class="flex flex-col gap-4">
		<!-- Game version display -->
		<div class="flex items-center justify-between rounded-lg bg-surface-2 border border-surface-5 px-4 py-2">
			<span class="text-sm text-secondary">{{ formatMessage(messages.gameVersionLabel) }}</span>
			<button
				class="font-semibold text-brand hover:underline"
				@click="ctx.modal.value?.setStage('game-version-select')"
			>
				{{ ctx.selectedGameVersion.value }}
			</button>
		</div>

		<!-- Loader cards -->
		<div class="flex flex-col gap-2">
			<div class="flex items-center justify-between">
				<span class="font-semibold text-contrast">{{ formatMessage(messages.selectLoader) }}</span>
				<span class="text-xs text-secondary">{{ formatMessage(messages.optionalHint) }}</span>
			</div>

			<!-- Vanilla (no loader) -->
			<div
				class="loader-card"
				:class="{ 'loader-card-selected': selectedLoaderId === 'vanilla' }"
				@click="selectLoader('vanilla')"
			>
				<div class="flex items-center gap-3 flex-1">
					<div class="loader-icon loader-icon-vanilla">
						<component :is="loaderIconMap['vanilla']" class="loader-icon-inner" />
					</div>
					<div class="flex flex-col">
						<span class="font-semibold text-contrast">Vanilla</span>
						<span class="text-xs text-secondary">{{ formatMessage(messages.vanillaDesc) }}</span>
					</div>
				</div>
				<CheckIcon v-if="selectedLoaderId === 'vanilla'" class="size-5 text-brand" />
			</div>

			<!-- Loader options -->
			<div
				v-for="opt in availableLoaderOptions"
				:key="opt.id"
				class="loader-card"
				:class="{ 'loader-card-selected': selectedLoaderId === opt.id, 'loader-card-disabled': !opt.available }"
				@click="opt.available && selectLoader(opt.id)"
			>
				<div class="flex items-center gap-3 flex-1">
					<div class="loader-icon" :class="`loader-icon-${opt.id}`">
						<component :is="loaderIconMap[opt.id] ?? TagLoaderModloaderIcon" class="loader-icon-inner" />
					</div>
					<div class="flex flex-col">
						<span class="font-semibold text-contrast">{{ opt.label }}</span>
						<span class="text-xs text-secondary">
							{{ opt.available
								? (opt.versionCount > 0
									? formatMessage(messages.versionsAvailable, { count: opt.versionCount })
									: formatMessage(messages.noVersions))
								: formatMessage(messages.incompatible) }}
						</span>
					</div>
				</div>
				<CheckIcon v-if="selectedLoaderId === opt.id" class="size-5 text-brand" />
			</div>
		</div>

		<!-- Loader version list (shown when a non-vanilla loader is selected) -->
		<div v-if="selectedLoaderId && selectedLoaderId !== 'vanilla' && loaderVersions.length > 0" class="flex flex-col gap-2">
			<div class="flex items-center justify-between">
				<span class="font-semibold text-contrast">{{ formatMessage(messages.loaderVersionLabel) }}</span>
				<Chips
					v-model="loaderVersionType"
					:items="loaderVersionTypeItems"
					:disabled-items="loaderVersionTypeDisabledItems"
					:format-label="formatLoaderVersionTypeLabel"
				/>
			</div>
			<div class="flex flex-col gap-1 max-h-[40vh] overflow-y-auto pr-1 -mr-1">
				<div
					v-for="v in loaderVersions"
					:key="v.id"
					class="loader-version-item"
					:class="{ 'loader-version-item-selected': ctx.selectedLoaderVersion.value === v.id }"
					@click="ctx.selectedLoaderVersion.value = v.id"
				>
					<div class="flex items-center gap-2 flex-1 min-w-0">
						<span class="font-semibold text-contrast truncate">{{ v.id }}</span>
						<span v-if="v.stable" class="text-xs px-1.5 py-0.5 rounded bg-green-highlight text-green">
							{{ formatMessage(messages.stable) }}
						</span>
					</div>
					<CheckIcon v-if="ctx.selectedLoaderVersion.value === v.id" class="size-4 text-brand" />
				</div>
			</div>
		</div>
		<div v-else-if="selectedLoaderId && selectedLoaderId !== 'vanilla' && loaderVersionsLoading" class="text-center text-secondary py-4">
			{{ formatMessage(commonMessages.loadingLabel) }}
		</div>
		<div v-else-if="selectedLoaderId && selectedLoaderId !== 'vanilla' && !loaderVersionsLoading && loaderVersions.length === 0" class="text-center text-secondary py-4">
			{{ formatMessage(messages.noVersions) }}
		</div>
	</div>
</template>

<script setup lang="ts">
import {
	CheckIcon,
	TagLoaderVanillaIcon,
	TagLoaderForgeIcon,
	TagLoaderFabricIcon,
	TagLoaderQuiltIcon,
	TagLoaderNeoforgeIcon,
	TagLoaderOptifineIcon,
	TagLoaderLiteloaderIcon,
	TagLoaderModloaderIcon,
	WrenchIcon,
	CubeIcon,
} from '@modrinth/assets'
import { commonMessages, defineMessages, useVIntl } from '@modrinth/ui'
import { type Component, computed, markRaw, onMounted, ref, watch } from 'vue'

import Chips from '../../../base/Chips.vue'
import type { LoaderVersionEntry, LoaderVersionType } from '../creation-flow-context'
import { injectCreationFlowContext } from '../creation-flow-context'
import { formatLoaderLabel } from '../shared'

const ctx = injectCreationFlowContext()
const { formatMessage } = useVIntl()

const loaderIconMap: Record<string, Component> = {
	vanilla: markRaw(TagLoaderVanillaIcon),
	forge: markRaw(TagLoaderForgeIcon),
	neoforge: markRaw(TagLoaderNeoforgeIcon),
	fabric: markRaw(TagLoaderFabricIcon),
	quilt: markRaw(TagLoaderQuiltIcon),
	optifine: markRaw(TagLoaderOptifineIcon),
	liteloader: markRaw(TagLoaderLiteloaderIcon),
	cleanroom: markRaw(WrenchIcon),
	labymod: markRaw(CubeIcon),
}

const messages = defineMessages({
	gameVersionLabel: {
		id: 'creation-flow.modal.loader-select.game-version',
		defaultMessage: 'Game version',
	},
	selectLoader: {
		id: 'creation-flow.modal.loader-select.title',
		defaultMessage: 'Select loader (optional)',
	},
	optionalHint: {
		id: 'creation-flow.modal.loader-select.optional',
		defaultMessage: 'Skip to play vanilla',
	},
	vanillaDesc: {
		id: 'creation-flow.modal.loader-select.vanilla.desc',
		defaultMessage: 'No mods, original Minecraft experience',
	},
	versionsAvailable: {
		id: 'creation-flow.modal.loader-select.versions-available',
		defaultMessage: '{count, plural, one {# version} other {# versions}} available',
	},
	noVersions: {
		id: 'creation-flow.modal.loader-select.no-versions',
		defaultMessage: 'No compatible versions',
	},
	incompatible: {
		id: 'creation-flow.modal.loader-select.incompatible',
		defaultMessage: 'Incompatible with this game version',
	},
	loaderVersionLabel: {
		id: 'creation-flow.modal.loader-select.loader-version',
		defaultMessage: 'Loader version',
	},
	stable: {
		id: 'creation-flow.modal.loader-select.stable',
		defaultMessage: 'Stable',
	},
	stableType: {
		id: 'creation-flow.modal.loader-select.type.stable',
		defaultMessage: 'Stable',
	},
	latestType: {
		id: 'creation-flow.modal.loader-select.type.latest',
		defaultMessage: 'Latest',
	},
	otherType: {
		id: 'creation-flow.modal.loader-select.type.other',
		defaultMessage: 'Other',
	},
})

// All loaders we want to support (mirrors PCL)
// We use ctx.availableLoaders (from the app) as the primary source,
// plus extras that don't have Modrinth manifest support but can be shown
interface LoaderOption {
	id: string
	label: string
	iconChar: string
}

// Map loader IDs to display info
const loaderInfoMap: Record<string, { label: string; iconChar: string }> = {
	forge: { label: 'Forge', iconChar: 'F' },
	neoforge: { label: 'NeoForge', iconChar: 'N' },
	fabric: { label: 'Fabric', iconChar: 'A' },
	quilt: { label: 'Quilt', iconChar: 'Q' },
	optifine: { label: 'OptiFine', iconChar: 'O' },
	liteloader: { label: 'LiteLoader', iconChar: 'L' },
	cleanroom: { label: 'Cleanroom', iconChar: 'C' },
	labymod: { label: 'LabyMod', iconChar: 'B' },
}

// Build the list from availableLoaders provided by the app
const allLoaders = computed<LoaderOption[]>(() => {
	return ctx.availableLoaders
		.filter((id) => id !== 'vanilla')
		.map((id) => {
			const info = loaderInfoMap[id] ?? { label: formatLoaderLabel(id), iconChar: id.charAt(0).toUpperCase() }
			return { id, ...info }
		})
})

const selectedLoaderId = ref<string | null>(null)
const loaderVersions = ref<LoaderVersionEntry[]>([])
const loaderVersionsLoading = ref(false)

// Loader version type
const loaderVersionType = ref<LoaderVersionType>('stable')
const loaderVersionTypeItems: LoaderVersionType[] = ['stable', 'latest', 'other']
const loaderVersionTypeDisabledItems = computed<LoaderVersionType[]>(() => {
	const noStable = !loaderVersions.value.some((v: LoaderVersionEntry) => v.stable)
	return noStable ? ['stable'] : []
})

function formatLoaderVersionTypeLabel(type: LoaderVersionType): string {
	if (type === 'stable') return formatMessage(messages.stableType)
	if (type === 'latest') return formatMessage(messages.latestType)
	return formatMessage(messages.otherType)
}

// Compute which loaders have versions for the selected game version
const availableLoaderOptions = computed(() => {
	const gameVersion = ctx.selectedGameVersion.value
	if (!gameVersion) return []

	return allLoaders.value.map((opt) => {
		// Check if we have manifest data
		const apiLoader = opt.id === 'neoforge' ? 'neo' : opt.id
		const manifest = ctx.loaderVersionsCache.value[apiLoader]

		let available = false
		let versionCount = 0

		if (manifest) {
			// Some loaders (e.g. Fabric) use a placeholder
			const placeholder = manifest.find((x) => x.id === '${modrinth.gameVersion}')
			if (placeholder) {
				// Check if this game version is in the loaders list
				const hasVersion = manifest.some((x) => x.id === gameVersion)
				available = hasVersion
				versionCount = available ? placeholder.loaders.length : 0
			} else {
				const entry = manifest.find((x) => x.id === gameVersion)
				available = !!entry
				versionCount = entry?.loaders.length ?? 0
			}
		}

		return {
			...opt,
			label: formatLoaderLabel(opt.id),
			available,
			versionCount,
		}
	})
})

// Watch loader version type
watch(loaderVersionType, () => autoSelectLoaderVersion())

function autoSelectLoaderVersion() {
	if (loaderVersionType.value === 'stable' && loaderVersionTypeDisabledItems.value.includes('stable')) {
		loaderVersionType.value = 'latest'
	}
	if (loaderVersionType.value === 'stable') {
		const stable = loaderVersions.value.find((v) => v.stable)
		ctx.selectedLoaderVersion.value = stable?.id ?? loaderVersions.value[0]?.id ?? null
	} else if (loaderVersionType.value === 'latest') {
		ctx.selectedLoaderVersion.value = loaderVersions.value[0]?.id ?? null
	} else if (loaderVersionType.value === 'other' && !ctx.selectedLoaderVersion.value) {
		ctx.selectedLoaderVersion.value = loaderVersions.value[0]?.id ?? null
	}
}

function selectLoader(loaderId: string) {
	selectedLoaderId.value = loaderId
	if (loaderId === 'vanilla') {
		ctx.selectedLoader.value = 'vanilla'
		ctx.selectedLoaderVersion.value = null
		loaderVersions.value = []
	} else {
		ctx.selectedLoader.value = loaderId
	}
}

// Fetch versions when loader changes
watch(selectedLoaderId, async (loaderId) => {
	if (!loaderId || loaderId === 'vanilla') {
		loaderVersions.value = []
		return
	}
	const gameVersion = ctx.selectedGameVersion.value
	if (!gameVersion) return

	loaderVersionsLoading.value = true
	loaderVersions.value = []
	ctx.selectedLoaderVersion.value = null

	await ctx.fetchLoaderMetadata(loaderId)

	const apiLoader = loaderId === 'neoforge' ? 'neo' : loaderId
	const manifest = ctx.loaderVersionsCache.value[apiLoader]
	if (!manifest) {
		loaderVersionsLoading.value = false
		return
	}

	// Placeholder (Fabric/Quilt pattern)
	const placeholder = manifest.find((x) => x.id === '${modrinth.gameVersion}')
	if (placeholder) {
		if (!manifest.some((x) => x.id === gameVersion)) {
			loaderVersionsLoading.value = false
			return
		}
		loaderVersions.value = placeholder.loaders
	} else {
		const entry = manifest.find((x) => x.id === gameVersion)
		loaderVersions.value = entry?.loaders ?? []
	}

	loaderVersionsLoading.value = false
	autoSelectLoaderVersion()
})

// On mount, default to vanilla
onMounted(() => {
	if (!ctx.selectedLoader.value) {
		selectedLoaderId.value = 'vanilla'
		ctx.selectedLoader.value = 'vanilla'
	} else {
		selectedLoaderId.value = ctx.selectedLoader.value
	}
	// Sync loaderVersionType from context
	loaderVersionType.value = ctx.loaderVersionType.value
})

// Sync back to context
watch(loaderVersionType, (v) => {
	ctx.loaderVersionType.value = v
})
</script>

<style scoped>
.loader-card {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding: 0.75rem 1rem;
	border-radius: 0.5rem;
	border: 1px solid var(--color-surface-5);
	background: var(--color-surface-2);
	cursor: pointer;
	transition: all 0.15s ease;
}

.loader-card:hover:not(.loader-card-disabled) {
	background: var(--color-surface-3);
	border-color: var(--color-brand);
}

.loader-card-selected {
	background: var(--color-brand-highlight);
	border-color: var(--color-brand);
}

.loader-card-disabled {
	opacity: 0.5;
	cursor: not-allowed;
}

.loader-icon {
	width: 2.5rem;
	height: 2.5rem;
	border-radius: 0.5rem;
	display: flex;
	align-items: center;
	justify-content: center;
	font-weight: 700;
	color: white;
	flex-shrink: 0;
}

.loader-icon-inner {
	width: 1.5rem;
	height: 1.5rem;
	flex-shrink: 0;
}

.loader-icon-vanilla { background: linear-gradient(135deg, #4caf50, #2e7d32); }
.loader-icon-forge { background: linear-gradient(135deg, #d97706, #92400e); }
.loader-icon-neoforge { background: linear-gradient(135deg, #f97316, #c2410c); }
.loader-icon-fabric { background: linear-gradient(135deg, #8b5cf6, #6d28d9); }
.loader-icon-quilt { background: linear-gradient(135deg, #ec4899, #be185d); }
.loader-icon-optifine { background: linear-gradient(135deg, #06b6d4, #0e7490); }
.loader-icon-liteloader { background: linear-gradient(135deg, #6b7280, #374151); }
.loader-icon-cleanroom { background: linear-gradient(135deg, #14b8a6, #0f766e); }
.loader-icon-labymod { background: linear-gradient(135deg, #3b82f6, #1d4ed8); }

.loader-version-item {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding: 0.5rem 0.75rem;
	border-radius: 0.375rem;
	border: 1px solid var(--color-surface-5);
	background: var(--color-surface-2);
	cursor: pointer;
	transition: all 0.15s ease;
}

.loader-version-item:hover {
	background: var(--color-surface-3);
}

.loader-version-item-selected {
	background: var(--color-brand-highlight);
	border-color: var(--color-brand);
}
</style>
