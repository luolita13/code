<script setup lang="ts">
import { CheckIcon, ModrinthIcon } from '@modrinth/assets'
import { ButtonStyled, NewModal } from '@modrinth/ui'
import { onMounted, ref } from 'vue'
import {
	compareVersions,
	fetchRemoteNotices,
	openExternalUpdateLink,
	type RemoteNotice,
} from '../../helpers/remote-config.ts'

const STORAGE_KEY_DISMISSED = 'modrinth-app-startup-notice-dismissed'
const STORAGE_KEY_SHOWN_NOTICES = 'modrinth-app-startup-shown-notice-ids'

const modal = ref<InstanceType<typeof NewModal> | null>(null)
const doNotShowAgain = ref(false)
const isLoading = ref(true)
const remoteNotice = ref<RemoteNotice | null>(null)

declare const __APP_VERSION__: string

function getCurrentAppVersion(): string {
	return typeof __APP_VERSION__ !== 'undefined' ? __APP_VERSION__ : '1.0.0'
}

function getShownNoticeIds(): string[] {
	try {
		return JSON.parse(localStorage.getItem(STORAGE_KEY_SHOWN_NOTICES) || '[]')
	} catch {
		return []
	}
}

function markNoticeShown(id: string) {
	const ids = new Set(getShownNoticeIds())
	ids.add(id)
	localStorage.setItem(STORAGE_KEY_SHOWN_NOTICES, JSON.stringify([...ids]))
}

function shouldShowNotice(notice: RemoteNotice): boolean {
	const appVersion = getCurrentAppVersion()

	if (notice.minAppVersion && compareVersions(appVersion, notice.minAppVersion) < 0) {
		return false
	}
	if (notice.maxAppVersion && compareVersions(appVersion, notice.maxAppVersion) > 0) {
		return false
	}

	if (notice.showOnce && getShownNoticeIds().includes(notice.id)) {
		return false
	}

	return true
}

onMounted(async () => {
	const dismissed = localStorage.getItem(STORAGE_KEY_DISMISSED)

	const noticesData = await fetchRemoteNotices()
	if (noticesData?.notices?.length) {
		remoteNotice.value = noticesData.notices.find(shouldShowNotice) ?? null
	}

	isLoading.value = false

	if (remoteNotice.value) {
		markNoticeShown(remoteNotice.value.id)
		modal.value?.show()
	} else if (dismissed !== 'true') {
		modal.value?.show()
	}
})

function dismiss() {
	if (!remoteNotice.value && doNotShowAgain.value) {
		localStorage.setItem(STORAGE_KEY_DISMISSED, 'true')
	}
	modal.value?.hide()
}

function handleLinkClick(event: MouseEvent) {
	const target = event.target as HTMLElement
	const anchor = target.closest('a')
	if (anchor?.href) {
		event.preventDefault()
		openExternalUpdateLink(anchor.href)
	}
}
</script>

<template>
	<NewModal
		ref="modal"
		:header="remoteNotice?.title || 'Welcome to this customized build of Modrinth App'"
		:closable="remoteNotice?.dismissible !== false"
		:close-on-click-outside="remoteNotice?.dismissible !== false"
		:close-on-esc="remoteNotice?.dismissible !== false"
		max-width="560px"
	>
		<div class="flex flex-col gap-4">
			<div v-if="!remoteNotice" class="flex items-center gap-3">
				<div class="p-3 rounded-xl bg-brand-highlight">
					<ModrinthIcon class="size-8 text-brand" />
				</div>
				<div>
					<p class="m-0 font-bold text-contrast text-lg">Modrinth App — Custom Edition</p>
					<p class="m-0 text-sm text-secondary">Unofficial modified distribution</p>
				</div>
			</div>

			<div
				class="text-sm text-primary leading-relaxed space-y-2"
				:class="{ 'max-h-[50vh] overflow-y-auto pr-1': remoteNotice?.body }"
				@click="handleLinkClick"
			>
				<template v-if="remoteNotice">
					<div class="whitespace-pre-wrap" v-html="remoteNotice.body" />
				</template>
				<template v-else>
					<p class="m-0">
						This application is a community-customized redistribution of the Modrinth App.
						It includes additional features and modifications that are not part of the
						official Modrinth release.
					</p>
					<p class="m-0">By continuing to use this software, you acknowledge that:</p>
					<ul class="m-0 pl-5 list-disc space-y-1">
						<li>
							This is an unofficial third-party build and is not affiliated with or endorsed
							by Modrinth / Rinth, Inc.
						</li>
						<li>
							All original Modrinth trademarks, assets, and code remain the property of their
							respective owners.
						</li>
						<li>
							Modifications are provided "as is" without warranty of any kind. Use at your
							own risk.
						</li>
						<li>
							This build may connect to Modrinth and other third-party services under their
							own terms of service and privacy policies.
						</li>
					</ul>
					<p class="m-0">
						Customized by
						<a
							href="https://github.com/luolita13"
							target="_blank"
							class="text-brand hover:underline"
						>
							github.com/luolita13
						</a>
						. Source and issues can be found there.
					</p>
				</template>
			</div>

			<div v-if="isLoading" class="text-center text-secondary text-sm py-2">
				Loading notice...
			</div>

			<div class="flex items-center justify-between pt-2">
				<button
					v-if="!remoteNotice"
					class="flex items-center gap-2 text-sm text-secondary hover:text-contrast transition-colors"
					@click="doNotShowAgain = !doNotShowAgain"
				>
					<span
						class="size-4 rounded border border-secondary flex items-center justify-center"
						:class="{ 'bg-brand border-brand': doNotShowAgain }"
					>
						<CheckIcon v-if="doNotShowAgain" class="size-3 text-white" />
					</span>
					Don't show again
				</button>
				<span v-else />

				<ButtonStyled type="brand">
					<button class="flex items-center gap-2" @click="dismiss">
						<svg
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path d="M20 6 9 17l-5-5" />
						</svg>
						{{ remoteNotice?.dismissible === false ? 'Close' : 'I understand and agree' }}
					</button>
				</ButtonStyled>
			</div>
		</div>
	</NewModal>
</template>
