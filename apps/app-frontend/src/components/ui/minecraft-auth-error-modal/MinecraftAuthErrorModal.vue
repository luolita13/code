<script setup lang="ts">
import {
	CheckIcon,
	CopyIcon,
	DropdownIcon,
	LogInIcon,
	MessagesSquareIcon,
	WrenchIcon,
} from '@modrinth/assets'
import { Admonition, ButtonStyled, Collapsible, defineMessages, NewModal, useVIntl } from '@modrinth/ui'
import { computed, ref } from 'vue'

import { hide_ads_window, show_ads_window } from '@/helpers/ads.js'
import { login as login_flow, set_default_user } from '@/helpers/auth.js'
import { handleSevereError } from '@/store/error.js'

import { findMinecraftAuthError, type MinecraftAuthError } from './minecraft-auth-errors'

const { formatMessage } = useVIntl()

const messages = defineMessages({
	header: { id: 'app.minecraft-auth.header', defaultMessage: 'Sign in Failed' },
	warning: {
		id: 'app.minecraft-auth.warning',
		defaultMessage:
			"We couldn't sign you into your Microsoft account. This may be due to account restrictions or regional limitations.",
	},
	whatHappened: { id: 'app.minecraft-auth.what-happened', defaultMessage: 'What we think happened' },
	howToFix: { id: 'app.minecraft-auth.how-to-fix', defaultMessage: 'How to fix it' },
	unknownError: { id: 'app.minecraft-auth.unknown-error', defaultMessage: 'Unknown error' },
	unknownErrorDesc: {
		id: 'app.minecraft-auth.unknown-error.desc',
		defaultMessage:
			"We don't recognize this error and can't recommend specific steps to resolve it.",
	},
	unknownErrorTry: {
		id: 'app.minecraft-auth.unknown-error.try',
		defaultMessage: 'Try visiting',
	},
	minecraftLogin: { id: 'app.minecraft-auth.minecraft-login', defaultMessage: 'Minecraft Login' },
	unknownErrorSupport: {
		id: 'app.minecraft-auth.unknown-error.support',
		defaultMessage:
			'and signing in, as it may prompt you with the necessary steps. You can also contact support and we can look into it further.',
	},
	contactSupport: { id: 'app.minecraft-auth.contact-support', defaultMessage: 'Contact support' },
	signInAgain: { id: 'app.minecraft-auth.sign-in-again', defaultMessage: 'Sign in again' },
	debugInfo: { id: 'app.minecraft-auth.debug-info', defaultMessage: 'Debug information' },
	copyDebugInfo: { id: 'app.minecraft-auth.copy-debug-info', defaultMessage: 'Copy debug info' },
	noErrorMessage: { id: 'app.minecraft-auth.no-error-message', defaultMessage: 'No error message.' },
})

const modal = ref<InstanceType<typeof NewModal>>()
const rawError = ref<string>('')
const matchedError = ref<MinecraftAuthError | null>(null)
const debugCollapsed = ref(true)
const copied = ref(false)
const loadingSignIn = ref(false)

function show(errorVal: { message?: string }) {
	rawError.value = errorVal?.message ?? String(errorVal)

	matchedError.value = findMinecraftAuthError(rawError.value)

	debugCollapsed.value = true
	hide_ads_window()
	modal.value?.show()
}

function hide() {
	onModalHide()
	modal.value?.hide()
}

function onModalHide() {
	show_ads_window()
}

defineExpose({
	show,
	hide,
})

async function signInAgain() {
	try {
		loadingSignIn.value = true
		const loggedIn = await login_flow()
		if (loggedIn) {
			await set_default_user(loggedIn.profile.id)
		}
		loadingSignIn.value = false
		modal.value?.hide()
	} catch (err) {
		loadingSignIn.value = false
		handleSevereError(err)
	}
}

const debugInfo = computed(() => rawError.value || formatMessage(messages.noErrorMessage))

async function copyToClipboard(text: string) {
	await navigator.clipboard.writeText(text)
	copied.value = true
	setTimeout(() => {
		copied.value = false
	}, 3000)
}
</script>

<template>
	<NewModal ref="modal" :header="formatMessage(messages.header)" :max-width="'548px'" @hide="onModalHide">
		<div class="flex flex-col gap-6">
			<Admonition
				type="warning"
				:body="formatMessage(messages.warning)"
			>
			</Admonition>

			<!-- Matched error details -->
			<div class="bg-surface-2 rounded-2xl p-4 px-5 flex flex-col gap-3">
				<template v-if="matchedError">
					<div class="flex flex-col gap-1.5">
						<h3 class="text-base font-bold m-0">{{ formatMessage(messages.whatHappened) }}</h3>
						<p class="text-sm text-secondary m-0">
							{{ matchedError.whatHappened }}
						</p>
					</div>

					<div class="flex flex-col gap-1.5">
						<h3 class="text-base font-bold m-0">{{ formatMessage(messages.howToFix) }}</h3>
						<ol class="list-none flex flex-col gap-2 m-0 pl-0">
							<li
								v-for="(step, index) in matchedError.stepsToFix"
								:key="index"
								class="flex items-baseline gap-2"
							>
								<span
									class="inline-flex items-center justify-center shrink-0 w-5 h-5 rounded-full bg-surface-4 border border-solid border-surface-5 text-xs font-medium"
								>
									{{ index + 1 }}
								</span>
								<!-- eslint-disable-next-line vue/no-v-html -->
								<span
									class="text-sm [&_a]:text-info [&_a]:font-medium [&_a]:underline"
									v-html="step"
								/>
							</li>
						</ol>
					</div>
				</template>
				<template v-else>
					<div class="flex flex-col gap-1.5">
						<h3 class="text-base font-bold m-0">{{ formatMessage(messages.unknownError) }}</h3>
						<p class="text-sm text-secondary m-0">
							{{ formatMessage(messages.unknownErrorDesc) }}
						</p>
						<p class="text-sm text-secondary m-0">
							{{ formatMessage(messages.unknownErrorTry) }}
							<a
								class="text-info font-medium underline hover:underline"
								href="https://www.minecraft.net/en-us/login"
								>{{ formatMessage(messages.minecraftLogin) }}</a
							>
							{{ formatMessage(messages.unknownErrorSupport) }}
						</p>
					</div>
				</template>
			</div>

			<!-- Action buttons -->
			<div class="flex items-center gap-2">
				<ButtonStyled>
					<a href="https://support.modrinth.com" class="!w-full" @click="modal?.hide()">
						<MessagesSquareIcon /> {{ formatMessage(messages.contactSupport) }}
					</a>
				</ButtonStyled>
				<ButtonStyled color="brand">
					<button :disabled="loadingSignIn" class="!w-full" @click="signInAgain">
						<LogInIcon /> {{ formatMessage(messages.signInAgain) }}
					</button>
				</ButtonStyled>
			</div>

			<div class="flex flex-col gap-2">
				<div class="w-full h-[1px] bg-surface-5"></div>

				<!-- Debug info -->
				<div class="overflow-clip">
					<button
						class="flex items-center justify-between w-full bg-transparent border-0 py-4 cursor-pointer"
						@click="debugCollapsed = !debugCollapsed"
					>
						<span class="flex items-center gap-2 text-contrast font-extrabold m-0">
							<WrenchIcon class="h-4 w-4" />
							{{ formatMessage(messages.debugInfo) }}
						</span>
						<DropdownIcon
							class="h-5 w-5 text-secondary transition-transform"
							:class="{ 'rotate-180': !debugCollapsed }"
						/>
					</button>
					<Collapsible :collapsed="debugCollapsed">
						<div
							class="p-3 bg-surface-2 rounded-2xl text-xs grid grid-cols-[1fr_auto] max-w-full items-start"
						>
							<div
								class="m-0 p-0 rounded-none bg-transparent text-sm font-mono break-words overflow-auto"
							>
								{{ debugInfo }}
							</div>
							<ButtonStyled circular>
								<button
									v-tooltip="formatMessage(messages.copyDebugInfo)"
									:disabled="copied"
									@click="copyToClipboard(debugInfo)"
								>
									<template v-if="copied"> <CheckIcon class="text-green" /> </template>
									<template v-else> <CopyIcon /> </template>
								</button>
							</ButtonStyled>
						</div>
					</Collapsible>
				</div>
			</div>
		</div>
	</NewModal>
</template>
