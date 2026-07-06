<script setup lang="ts">
import { ButtonStyled, NewModal } from '@modrinth/ui'
import { ref } from 'vue'
import { compareVersions, fetchRemoteVersion, openExternalUpdateLink } from '../../helpers/remote-config.ts'

declare const __APP_VERSION__: string

const modal = ref<InstanceType<typeof NewModal> | null>(null)
const latestVersion = ref('')
const downloadUrl = ref('')
const releaseNotes = ref('')

async function checkForUpdate() {
	const currentVersion = typeof __APP_VERSION__ !== 'undefined' ? __APP_VERSION__ : '1.0.0'
	const remote = await fetchRemoteVersion()
	if (!remote?.latest) return false

	const remoteVersion = remote.latest.replace(/^v/, '')
	if (compareVersions(remoteVersion, currentVersion) > 0) {
		latestVersion.value = remoteVersion
		downloadUrl.value = remote.downloadUrl
		releaseNotes.value = remote.releaseNotes || ''
		modal.value?.show()
		return true
	}
	return false
}

function openDownload() {
	if (downloadUrl.value) {
		openExternalUpdateLink(downloadUrl.value)
	}
}

defineExpose({ checkForUpdate })
</script>

<template>
	<NewModal ref="modal" header="Update available" max-width="480px">
		<div class="flex flex-col gap-4">
			<p class="m-0 text-primary">
				A new version <span class="font-bold text-contrast">{{ latestVersion }}</span> is
				available.
			</p>
			<div
				v-if="releaseNotes"
				class="text-sm text-primary bg-bg-raised p-3 rounded-xl max-h-[30vh] overflow-y-auto whitespace-pre-wrap"
			>
				{{ releaseNotes }}
			</div>
			<div class="flex justify-end gap-2">
				<ButtonStyled type="secondary">
					<button @click="modal?.hide()">Later</button>
				</ButtonStyled>
				<ButtonStyled type="brand">
					<button @click="openDownload">Download update</button>
				</ButtonStyled>
			</div>
		</div>
	</NewModal>
</template>
