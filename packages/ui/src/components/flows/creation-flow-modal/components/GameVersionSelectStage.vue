<template>
	<div class="flex flex-col gap-4">
		<!-- Search bar -->
		<StyledInput
			v-model="searchQuery"
			:placeholder="formatMessage(messages.searchPlaceholder)"
			type="text"
		>
			<template #icon>
				<SearchIcon />
			</template>
		</StyledInput>

		<!-- Snapshot toggle -->
		<div class="flex items-center justify-between">
			<span class="text-sm text-secondary">{{ formatMessage(messages.showAllVersions) }}</span>
			<Toggle v-model="ctx.showSnapshots.value" />
		</div>

		<!-- Version list -->
		<div class="flex flex-col gap-3 max-h-[50vh] overflow-y-auto pr-1 -mr-1">
			<template v-for="group in versionGroups" :key="group.id">
				<div v-if="group.versions.length > 0" class="flex flex-col gap-2">
					<div class="flex items-center gap-2 sticky top-0 z-10 bg-bg-raised py-1">
						<div class="version-group-icon" :class="`version-group-icon-${group.id}`">
							<TagLoaderMinecraftIcon class="version-group-icon-inner" />
						</div>
						<span class="text-sm font-semibold text-contrast">{{ group.label }}</span>
						<span class="text-xs text-secondary">({{ group.versions.length }})</span>
					</div>
					<div
						v-for="v in group.versions"
						:key="v.version"
						class="version-item"
						:class="{ 'version-item-selected': isSelected(v.version) }"
						@click="selectVersion(v.version)"
					>
						<div class="flex items-center gap-3 flex-1 min-w-0">
							<div class="version-icon" :class="`version-icon-${group.id}`">
								<TagLoaderMinecraftIcon class="version-icon-inner" />
							</div>
							<div class="flex flex-col min-w-0">
								<span class="font-semibold text-contrast truncate">{{ v.version }}</span>
								<span v-if="v.date" class="text-xs text-secondary">
									{{ formatDate(v.date) }}
								</span>
							</div>
						</div>
						<div v-if="isSelected(v.version)" class="text-brand">
							<CheckIcon class="size-5" />
						</div>
					</div>
				</div>
			</template>
			<div v-if="filteredVersions.length === 0" class="text-center text-secondary py-8">
				{{ formatMessage(messages.noVersionsFound) }}
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { CheckIcon, SearchIcon, TagLoaderMinecraftIcon } from '@modrinth/assets'
import { defineMessages, useVIntl } from '@modrinth/ui'
import { computed, ref } from 'vue'

import { injectTags } from '../../../../providers'
import StyledInput from '../../../base/StyledInput.vue'
import Toggle from '../../../base/Toggle.vue'
import { injectCreationFlowContext } from '../creation-flow-context'

const { formatMessage } = useVIntl()
const ctx = injectCreationFlowContext()
const tags = injectTags()

const messages = defineMessages({
	searchPlaceholder: {
		id: 'creation-flow.modal.game-version-select.search',
		defaultMessage: 'Search version...',
	},
	showAllVersions: {
		id: 'creation-flow.modal.game-version-select.show-all',
		defaultMessage: 'Show all versions (snapshots, beta, alpha)',
	},
	noVersionsFound: {
		id: 'creation-flow.modal.game-version-select.no-results',
		defaultMessage: 'No versions found',
	},
	recentRelease: {
		id: 'creation-flow.modal.game-version-select.group.recent-release',
		defaultMessage: 'Recent releases',
	},
	release: {
		id: 'creation-flow.modal.game-version-select.group.release',
		defaultMessage: 'Releases',
	},
	snapshot: {
		id: 'creation-flow.modal.game-version-select.group.snapshot',
		defaultMessage: 'Snapshots',
	},
	oldBeta: {
		id: 'creation-flow.modal.game-version-select.group.old-beta',
		defaultMessage: 'Old Beta',
	},
	oldAlpha: {
		id: 'creation-flow.modal.game-version-select.group.old-alpha',
		defaultMessage: 'Old Alpha',
	},
})

const searchQuery = ref('')

interface VersionGroup {
	id: string
	label: string
	versions: { version: string; date?: string; version_type: string }[]
}

// Categorize similar to PCL: recent release first, then releases, snapshots, beta, alpha
const versionGroups = computed<VersionGroup[]>(() => {
	const all = ctx.showSnapshots.value
		? tags.gameVersions.value
		: tags.gameVersions.value.filter((v) => v.version_type === 'release')

	const q = searchQuery.value.trim().toLowerCase()
	const filtered = q
		? all.filter((v) => v.version.toLowerCase().includes(q))
		: all

	// Sort by date descending
	const sorted = [...filtered].sort((a, b) => {
		const ta = a.date ? Date.parse(a.date) : 0
		const tb = b.date ? Date.parse(b.date) : 0
		return tb - ta
	})

	// Group: latest release separated from older releases
	const releases = sorted.filter((v) => v.version_type === 'release')
	const snapshots = sorted.filter((v) => v.version_type === 'snapshot')
	const oldBeta = sorted.filter((v) => v.version_type === 'old_beta')
	const oldAlpha = sorted.filter((v) => v.version_type === 'old_alpha')

	const recentRelease = releases.slice(0, 3)
	const otherReleases = releases.slice(3)

	return [
		{ id: 'recent', label: formatMessage(messages.recentRelease), versions: recentRelease },
		{ id: 'release', label: formatMessage(messages.release), versions: otherReleases },
		{ id: 'snapshot', label: formatMessage(messages.snapshot), versions: snapshots },
		{ id: 'old-beta', label: formatMessage(messages.oldBeta), versions: oldBeta },
		{ id: 'old-alpha', label: formatMessage(messages.oldAlpha), versions: oldAlpha },
	]
})

const filteredVersions = computed(() =>
	versionGroups.value.flatMap((g) => g.versions),
)

function isSelected(version: string): boolean {
	return ctx.selectedGameVersion.value === version
}

function selectVersion(version: string) {
	ctx.selectedGameVersion.value = version
}

function formatDate(dateStr: string): string {
	try {
		const d = new Date(dateStr)
		return d.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' })
	} catch {
		return ''
	}
}
</script>

<style scoped>
.version-item {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding: 0.5rem 0.75rem;
	border-radius: 0.5rem;
	border: 1px solid var(--color-surface-5);
	background: var(--color-surface-2);
	cursor: pointer;
	transition: all 0.15s ease;
}

.version-item:hover {
	background: var(--color-surface-3);
	border-color: var(--color-brand);
}

.version-item-selected {
	background: var(--color-brand-highlight);
	border-color: var(--color-brand);
}

.version-icon {
	width: 2rem;
	height: 2rem;
	border-radius: 0.375rem;
	display: flex;
	align-items: center;
	justify-content: center;
	font-weight: 700;
	color: white;
	flex-shrink: 0;
}

.version-icon-inner {
	width: 1.25rem;
	height: 1.25rem;
	flex-shrink: 0;
}

.version-icon-recent {
	background: linear-gradient(135deg, #00b884, #008660);
}

.version-icon-release {
	background: linear-gradient(135deg, #1bd96a, #0f9c47);
}

.version-icon-snapshot {
	background: linear-gradient(135deg, #ff9a3c, #ff6b00);
}

.version-icon-old-beta {
	background: linear-gradient(135deg, #6b7280, #374151);
}

.version-icon-old-alpha {
	background: linear-gradient(135deg, #9ca3af, #4b5563);
}

.version-group-icon {
	width: 1.5rem;
	height: 1.5rem;
	border-radius: 0.375rem;
	display: flex;
	align-items: center;
	justify-content: center;
	color: white;
	flex-shrink: 0;
}

.version-group-icon-inner {
	width: 1rem;
	height: 1rem;
	flex-shrink: 0;
}

.version-group-icon-recent { background: linear-gradient(135deg, #00b884, #008660); }
.version-group-icon-release { background: linear-gradient(135deg, #1bd96a, #0f9c47); }
.version-group-icon-snapshot { background: linear-gradient(135deg, #ff9a3c, #ff6b00); }
.version-group-icon-old-beta { background: linear-gradient(135deg, #6b7280, #374151); }
.version-group-icon-old-alpha { background: linear-gradient(135deg, #9ca3af, #4b5563); }
</style>
