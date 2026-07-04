<template>
	<div v-if="instance" class="flex flex-col gap-6">
		<ExportModal ref="exportModal" :instance="instance" />
		<ConfirmDeleteInstanceModal ref="deleteInstanceModal" @delete="doDeleteInstance" />
		<!-- Instance information card -->
		<Card>
			<template #header>
				<h2 class="text-xl font-bold text-contrast m-0">
					{{ formatMessage(messages.information) }}
				</h2>
			</template>
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-2">
				<div class="info-item">
					<span class="info-label">{{ formatMessage(messages.gameVersion) }}</span>
					<span class="info-value">{{ instance.game_version }}</span>
				</div>
				<div class="info-item">
					<span class="info-label">{{ formatMessage(messages.loader) }}</span>
					<span class="info-value capitalize">{{ instance.loader }}</span>
				</div>
				<div v-if="instance.loader_version" class="info-item">
					<span class="info-label">{{ formatMessage(messages.loaderVersion) }}</span>
					<span class="info-value truncate" :title="instance.loader_version">
						{{ instance.loader_version }}
					</span>
				</div>
				<div class="info-item">
					<span class="info-label">{{ formatMessage(messages.installStage) }}</span>
					<span class="info-value" :class="installStageClass">
						{{ formatInstallStage(instance.install_stage) }}
					</span>
				</div>
				<div class="info-item">
					<span class="info-label">{{ formatMessage(messages.created) }}</span>
					<span class="info-value">{{ formatDate(instance.created) }}</span>
				</div>
				<div v-if="instance.last_played" class="info-item">
					<span class="info-label">{{ formatMessage(messages.lastPlayed) }}</span>
					<span class="info-value">{{ formatDate(instance.last_played) }}</span>
				</div>
				<div class="info-item">
					<span class="info-label">{{ formatMessage(messages.playTime) }}</span>
					<span class="info-value">{{ formatPlayTime }}</span>
				</div>
				<div v-if="instance.link?.type" class="info-item">
					<span class="info-label">{{ formatMessage(messages.linkType) }}</span>
					<span class="info-value">{{ formatLinkType(instance.link.type) }}</span>
				</div>
				<div v-if="instance.update_channel" class="info-item">
					<span class="info-label">{{ formatMessage(messages.updateChannel) }}</span>
					<span class="info-value capitalize">{{ instance.update_channel }}</span>
				</div>
			</div>
		</Card>

		<!-- Advanced management -->
		<Card>
			<template #header>
				<h2 class="text-xl font-bold text-contrast m-0">
					{{ formatMessage(messages.advancedManagement) }}
				</h2>
			</template>
			<div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3 p-2">
				<button
					v-for="action in advancedActions"
					:key="action.id"
					class="action-card"
					:class="{ 'action-card-danger': action.danger }"
					:disabled="action.disabled"
					@click="action.action"
				>
					<component :is="action.icon" class="size-6" :class="action.danger ? 'text-red' : 'text-brand'" />
					<span class="text-sm font-medium text-contrast">{{ action.label }}</span>
				</button>
			</div>
		</Card>
	</div>
</template>

<script setup lang="ts">
import {
	CopyIcon,
	DownloadIcon,
	PackageIcon,
	RefreshCwIcon,
	TrashIcon,
} from '@modrinth/assets'
import { Card, defineMessages, injectNotificationManager, useVIntl } from '@modrinth/ui'
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import { computed, ref, useTemplateRef } from 'vue'

import ConfirmDeleteInstanceModal from '@/components/ui/modal/ConfirmDeleteInstanceModal.vue'
import type ContextMenu from '@/components/ui/ContextMenu.vue'
import ExportModal from '@/components/ui/ExportModal.vue'
import { install_existing_instance } from '@/helpers/install'
import { remove as removeInstance } from '@/helpers/instance'
import type { GameInstance } from '@/helpers/types'
import { createInstanceShortcut } from '@/helpers/utils.js'

dayjs.extend(relativeTime)

const { formatMessage } = useVIntl()
const { addNotification, handleError } = injectNotificationManager()

const messages = defineMessages({
	information: { id: 'app.instance.overview.information', defaultMessage: 'Information' },
	gameVersion: { id: 'app.instance.overview.game-version', defaultMessage: 'Game version' },
	loader: { id: 'app.instance.overview.loader', defaultMessage: 'Loader' },
	loaderVersion: { id: 'app.instance.overview.loader-version', defaultMessage: 'Loader version' },
	installStage: { id: 'app.instance.overview.install-stage', defaultMessage: 'Status' },
	created: { id: 'app.instance.overview.created', defaultMessage: 'Created' },
	lastPlayed: { id: 'app.instance.overview.last-played', defaultMessage: 'Last played' },
	playTime: { id: 'app.instance.overview.play-time', defaultMessage: 'Play time' },
	linkType: { id: 'app.instance.overview.link-type', defaultMessage: 'Source' },
	updateChannel: { id: 'app.instance.overview.update-channel', defaultMessage: 'Update channel' },
	advancedManagement: {
		id: 'app.instance.overview.advanced-management',
		defaultMessage: 'Advanced management',
	},
	exportModpack: { id: 'app.instance.overview.export-modpack', defaultMessage: 'Export modpack' },
	repair: { id: 'app.instance.overview.repair', defaultMessage: 'Repair instance' },
	createShortcut: { id: 'app.instance.overview.create-shortcut', defaultMessage: 'Create shortcut' },
	copyInstance: { id: 'app.instance.overview.copy-instance', defaultMessage: 'Copy instance' },
	deleteInstance: { id: 'app.instance.overview.delete-instance', defaultMessage: 'Delete instance' },
	installed: { id: 'app.instance.overview.status.installed', defaultMessage: 'Installed' },
	notInstalled: { id: 'app.instance.overview.status.not-installed', defaultMessage: 'Not installed' },
	installing: { id: 'app.instance.overview.status.installing', defaultMessage: 'Installing' },
	packInstalled: { id: 'app.instance.overview.status.pack-installed', defaultMessage: 'Modpack installed' },
	packInstalling: {
		id: 'app.instance.overview.status.pack-installing',
		defaultMessage: 'Modpack installing',
	},
	minecraftInstalling: {
		id: 'app.instance.overview.status.minecraft-installing',
		defaultMessage: 'Minecraft installing',
	},
})

const props = defineProps<{
	instance: GameInstance
	options: InstanceType<typeof ContextMenu>
	offline: boolean
	playing: boolean
	installed: boolean
	isServerInstance?: boolean
	openSettings?: () => void
}>()

const emit = defineEmits<{
	(event: 'play'): void
	(event: 'stop'): void
}>()

const instance = computed(() => props.instance)
const exportModal = ref<InstanceType<typeof ExportModal>>()

const formatPlayTime = computed(() => {
	const total = instance.value
		? instance.value.recent_time_played + instance.value.submitted_time_played
		: 0
	if (total <= 0) return 'Never played'
	const hours = Math.floor(total / 3600)
	const minutes = Math.floor((total % 3600) / 60)
	if (hours >= 1) return `${hours}h ${minutes}m`
	return `${minutes}m`
})

const installStageClass = computed(() => {
	const stage = instance.value?.install_stage
	if (stage === 'installed') return 'text-green'
	if (stage === 'not_installed') return 'text-red'
	return 'text-orange'
})

function formatDate(date: Date | string): string {
	if (!date) return '—'
	return dayjs(date).format('YYYY-MM-DD HH:mm')
}

function formatInstallStage(stage: string): string {
	const map: Record<string, typeof messages.installed> = {
		installed: messages.installed,
		not_installed: messages.notInstalled,
		minecraft_installing: messages.minecraftInstalling,
		pack_installed: messages.packInstalled,
		pack_installing: messages.packInstalling,
	}
	return formatMessage(map[stage] ?? messages.installed)
}

function formatLinkType(type: string): string {
	const labels: Record<string, string> = {
		modrinth_modpack: 'Modrinth modpack',
		server_project: 'Server project',
		server_project_modpack: 'Server modpack',
		imported_modpack: 'Imported modpack',
		moderation_hosting: 'Hosting',
		shared_instance: 'Shared instance',
	}
	return labels[type] ?? type
}

async function repairInstance() {
	try {
		await install_existing_instance(instance.value.id, false)
		addNotification({ type: 'success', title: 'Instance repaired' })
	} catch (err) {
		handleError(err as Error)
	}
}

async function createShortcut() {
	try {
		const shortcutPath = await createInstanceShortcut(instance.value.name, instance.value.id)
		if (shortcutPath) {
			addNotification({ type: 'success', title: 'Shortcut created' })
		}
	} catch (err) {
		handleError(err as Error)
	}
}

const deleteInstanceModal = useTemplateRef('deleteInstanceModal')

async function deleteInstance() {
	deleteInstanceModal.value?.show()
}

async function doDeleteInstance() {
	try {
		await removeInstance(instance.value.id)
		addNotification({ type: 'success', title: 'Instance deleted' })
	} catch (err) {
		handleError(err as Error)
	}
}

const advancedActions = computed(() => [
	{
		id: 'export',
		label: formatMessage(messages.exportModpack),
		icon: PackageIcon,
		action: () => exportModal.value?.show(),
		disabled: false,
		danger: false,
	},
	{
		id: 'repair',
		label: formatMessage(messages.repair),
		icon: RefreshCwIcon,
		action: repairInstance,
		disabled: props.installed,
		danger: false,
	},
	{
		id: 'shortcut',
		label: formatMessage(messages.createShortcut),
		icon: DownloadIcon,
		action: createShortcut,
		disabled: false,
		danger: false,
	},
	{
		id: 'copy',
		label: formatMessage(messages.copyInstance),
		icon: CopyIcon,
		action: () => props.openSettings?.(),
		disabled: false,
		danger: false,
	},
	{
		id: 'delete',
		label: formatMessage(messages.deleteInstance),
		icon: TrashIcon,
		action: deleteInstance,
		disabled: false,
		danger: true,
	},
])
</script>

<style scoped lang="scss">
.info-item {
	display: flex;
	flex-direction: column;
	gap: 0.25rem;
	padding: 0.5rem 0.75rem;
	border-radius: 0.5rem;
	background: var(--color-surface-2);
	border: 1px solid var(--color-surface-5);
}

.info-label {
	font-size: 0.75rem;
	color: var(--color-secondary);
	text-transform: uppercase;
	letter-spacing: 0.05em;
}

.info-value {
	font-size: 0.95rem;
	font-weight: 600;
	color: var(--color-contrast);
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
}

.action-card {
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	gap: 0.5rem;
	padding: 1rem;
	border-radius: 0.5rem;
	background: var(--color-surface-2);
	border: 1px solid var(--color-surface-5);
	cursor: pointer;
	transition: all 0.15s ease;
	text-align: center;
}

.action-card:hover:not(:disabled) {
	background: var(--color-surface-3);
	border-color: var(--color-brand);
	transform: translateY(-1px);
}

.action-card:disabled {
	opacity: 0.5;
	cursor: not-allowed;
}

.action-card-danger:hover:not(:disabled) {
	border-color: var(--color-red);
}
</style>
