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

	</div>
</template>

<script setup lang="ts">
import { Card, defineMessages, StyledInput, Toggle, useVIntl } from '@modrinth/ui'
import { ref } from 'vue'

import { injectInstanceSettings } from '@/providers/instance-settings'

const { formatMessage } = useVIntl()
const ctx = injectInstanceSettings()

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
})

// Local state (these would be persisted via edit() in a full implementation)
const windowTitle = ref('')
const ignoreJavaCompat = ref(false)
const disableFileVerify = ref(false)
const useProxy = ref(false)
const proxyUrl = ref('')

</script>
