<template>
	<div class="flex flex-col gap-4" @contextmenu.prevent.stop>
		<!-- Game window title -->
		<Card>
			<template #header>
				<h3 class="text-lg font-semibold text-contrast m-0">
					{{ formatMessage(messages.windowTitle) }}
				</h3>
			</template>
			<div class="p-2">
				<StyledInput
					v-model="windowTitle"
					type="text"
					:placeholder="formatMessage(messages.windowTitlePlaceholder)"
					clearable
					wrapper-class="w-full"
				/>
				<p class="text-xs text-secondary mt-1">
					{{ formatMessage(messages.windowTitleHint) }}
				</p>
			</div>
		</Card>

		<!-- Advanced launch options -->
		<Card>
			<template #header>
				<h3 class="text-lg font-semibold text-contrast m-0">
					{{ formatMessage(messages.advancedOptions) }}
				</h3>
			</template>
			<div class="flex flex-col gap-3 p-2">
				<label class="flex items-center justify-between gap-3 cursor-pointer">
					<span class="text-sm text-contrast">{{ formatMessage(messages.ignoreJavaCompat) }}</span>
					<Toggle v-model="ignoreJavaCompat" />
				</label>
				<label class="flex items-center justify-between gap-3 cursor-pointer">
					<span class="text-sm text-contrast">{{ formatMessage(messages.disableFileVerify) }}</span>
					<Toggle v-model="disableFileVerify" />
				</label>
				<label class="flex items-center justify-between gap-3 cursor-pointer">
					<span class="text-sm text-contrast">{{ formatMessage(messages.useProxy) }}</span>
					<Toggle v-model="useProxy" />
				</label>
				<div v-if="useProxy" class="pl-6">
					<StyledInput
						v-model="proxyUrl"
						type="text"
						:placeholder="formatMessage(messages.proxyPlaceholder)"
						wrapper-class="w-full"
					/>
				</div>
			</div>
		</Card>

		<!-- Quick actions -->
		<Card>
			<template #header>
				<h3 class="text-lg font-semibold text-contrast m-0">
					{{ formatMessage(messages.quickManagement) }}
				</h3>
			</template>
			<div class="grid grid-cols-2 gap-3 p-2">
				<ButtonStyled type="outlined">
					<button class="!h-10 flex items-center justify-center gap-2" @click="resetInstance">
						<RefreshCwIcon class="size-4" />
						{{ formatMessage(messages.resetInstance) }}
					</button>
				</ButtonStyled>
				<ButtonStyled type="outlined" color="red">
					<button class="!h-10 flex items-center justify-center gap-2" @click="deleteInstance">
						<TrashIcon class="size-4" />
						{{ formatMessage(messages.deleteInstance) }}
					</button>
				</ButtonStyled>
			</div>
		</Card>

		<ConfirmModalWrapper
			ref="resetConfirmModal"
			:title="formatMessage(messages.resetInstance)"
			:description="formatMessage(messages.confirmReset)"
			:proceed-label="formatMessage(messages.resetInstance)"
			@proceed="doResetInstance"
		/>
		<ConfirmModalWrapper
			ref="deleteConfirmModal"
			:title="formatMessage(messages.deleteInstance)"
			:description="formatMessage(messages.confirmDelete)"
			:proceed-label="formatMessage(messages.deleteInstance)"
			@proceed="doDeleteInstance"
		/>
	</div>
</template>

<script setup lang="ts">
import { RefreshCwIcon, TrashIcon } from '@modrinth/assets'
import { ButtonStyled, Card, defineMessages, injectNotificationManager, StyledInput, Toggle, useVIntl } from '@modrinth/ui'
import { ref, useTemplateRef } from 'vue'

import ConfirmModalWrapper from '@/components/ui/modal/ConfirmModalWrapper.vue'
import { remove as removeInstance } from '@/helpers/instance'
import { install_existing_instance } from '@/helpers/install'
import { injectInstanceSettings } from '@/providers/instance-settings'

const { formatMessage } = useVIntl()
const { addNotification, handleError } = injectNotificationManager()
const ctx = injectInstanceSettings()

const resetConfirmModal = useTemplateRef('resetConfirmModal')
const deleteConfirmModal = useTemplateRef('deleteConfirmModal')

const messages = defineMessages({
	windowTitle: { id: 'instance.settings.advanced.window-title', defaultMessage: 'Game window title' },
	windowTitlePlaceholder: {
		id: 'instance.settings.advanced.window-title-placeholder',
		defaultMessage: 'Minecraft* - {instance}',
	},
	windowTitleHint: {
		id: 'instance.settings.advanced.window-title-hint',
		defaultMessage: 'Leave empty to use the default title.',
	},
	advancedOptions: {
		id: 'instance.settings.advanced.options',
		defaultMessage: 'Advanced launch options',
	},
	ignoreJavaCompat: {
		id: 'instance.settings.advanced.ignore-java-compat',
		defaultMessage: 'Ignore Java compatibility',
	},
	disableFileVerify: {
		id: 'instance.settings.advanced.disable-file-verify',
		defaultMessage: 'Disable file verification',
	},
	useProxy: { id: 'instance.settings.advanced.use-proxy', defaultMessage: 'Use proxy' },
	proxyPlaceholder: {
		id: 'instance.settings.advanced.proxy-placeholder',
		defaultMessage: 'http://host:port',
	},
	quickManagement: {
		id: 'instance.settings.advanced.quick-management',
		defaultMessage: 'Quick management',
	},
	resetInstance: {
		id: 'instance.settings.advanced.reset-instance',
		defaultMessage: 'Reset instance',
	},
	deleteInstance: {
		id: 'instance.settings.advanced.delete-instance',
		defaultMessage: 'Delete instance',
	},
	confirmReset: {
		id: 'instance.settings.advanced.confirm-reset',
		defaultMessage: 'Reset this instance? Core files will be reinstalled.',
	},
	confirmDelete: {
		id: 'instance.settings.advanced.confirm-delete',
		defaultMessage: 'Delete this instance permanently? This cannot be undone.',
	},
	resetDone: { id: 'instance.settings.advanced.reset-done', defaultMessage: 'Instance reset' },
	deleted: { id: 'instance.settings.advanced.deleted', defaultMessage: 'Instance deleted' },
})

// Local state (these would be persisted via edit() in a full implementation)
const windowTitle = ref('')
const ignoreJavaCompat = ref(false)
const disableFileVerify = ref(false)
const useProxy = ref(false)
const proxyUrl = ref('')

function resetInstance() {
	resetConfirmModal.value?.show()
}

async function doResetInstance() {
	try {
		await install_existing_instance(ctx.instance.value.id, true)
		addNotification({ type: 'success', title: formatMessage(messages.resetDone) })
	} catch (err) {
		handleError(err as Error)
	}
}

function deleteInstance() {
	deleteConfirmModal.value?.show()
}

async function doDeleteInstance() {
	try {
		await removeInstance(ctx.instance.value.id)
		addNotification({ type: 'success', title: formatMessage(messages.deleted) })
		ctx.closeModal?.()
	} catch (err) {
		handleError(err as Error)
	}
}
</script>
