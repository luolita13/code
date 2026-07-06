<script setup lang="ts">
import {
	Combobox,
	defineMessages,
	injectNotificationManager,
	Slider,
	StyledInput,
	Toggle,
	useVIntl,
} from '@modrinth/ui'
import { computed, ref, watch } from 'vue'

import useMemorySlider from '@/composables/useMemorySlider'
import { get_max_memory } from '@/helpers/jre.js'
import { get, set } from '@/helpers/settings.ts'

const { handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	fullscreenTitle: {
		id: 'app.default-instance.fullscreen.title',
		defaultMessage: 'Fullscreen',
	},
	fullscreenDescription: {
		id: 'app.default-instance.fullscreen.description',
		defaultMessage: 'Overwrites the options.txt file to start in full screen when launched.',
	},
	widthTitle: {
		id: 'app.default-instance.width.title',
		defaultMessage: 'Width',
	},
	widthDescription: {
		id: 'app.default-instance.width.description',
		defaultMessage: 'The width of the game window when launched.',
	},
	widthPlaceholder: {
		id: 'app.default-instance.width.placeholder',
		defaultMessage: 'Enter width...',
	},
	heightTitle: {
		id: 'app.default-instance.height.title',
		defaultMessage: 'Height',
	},
	heightDescription: {
		id: 'app.default-instance.height.description',
		defaultMessage: 'The height of the game window when launched.',
	},
	heightPlaceholder: {
		id: 'app.default-instance.height.placeholder',
		defaultMessage: 'Enter height...',
	},
	memoryAllocationTitle: {
		id: 'app.default-instance.memory-allocation.title',
		defaultMessage: 'Memory allocation',
	},
	memoryAllocationDescription: {
		id: 'app.default-instance.memory-allocation.description',
		defaultMessage: 'Choose how memory is allocated to instances.',
	},
	memoryModeAuto: {
		id: 'app.default-instance.memory-mode.auto',
		defaultMessage: 'Auto',
	},
	memoryModeCustom: {
		id: 'app.default-instance.memory-mode.custom',
		defaultMessage: 'Custom',
	},
	memoryAllocated: {
		id: 'app.default-instance.memory-allocated',
		defaultMessage: 'Memory allocated',
	},
	memoryWarning: {
		id: 'app.default-instance.memory-warning',
		defaultMessage:
			'You are allocating more than 75% of your system memory to Minecraft. This may cause instability. Consider using "Auto" mode.',
	},
	memoryAutoHint: {
		id: 'app.default-instance.memory-auto-hint',
		defaultMessage:
			'Memory will be automatically calculated based on system RAM, instance type, and mod count.',
	},
	launchOptionsTitle: {
		id: 'app.default-instance.launch-options.title',
		defaultMessage: 'Launch options',
	},
	processPriorityTitle: {
		id: 'app.default-instance.process-priority.title',
		defaultMessage: 'Process priority',
	},
	processPriorityDescription: {
		id: 'app.default-instance.process-priority.description',
		defaultMessage: 'Set the CPU priority of the game process.',
	},
	priorityNormal: {
		id: 'app.default-instance.priority.normal',
		defaultMessage: 'Normal',
	},
	priorityAboveNormal: {
		id: 'app.default-instance.priority.above-normal',
		defaultMessage: 'Above normal',
	},
	priorityHigh: {
		id: 'app.default-instance.priority.high',
		defaultMessage: 'High',
	},
	priorityBelowNormal: {
		id: 'app.default-instance.priority.below-normal',
		defaultMessage: 'Below normal',
	},
	priorityRealtime: {
		id: 'app.default-instance.priority.realtime',
		defaultMessage: 'Realtime',
	},
	ipProtocolTitle: {
		id: 'app.default-instance.ip-protocol.title',
		defaultMessage: 'IP protocol preference',
	},
	ipProtocolDescription: {
		id: 'app.default-instance.ip-protocol.description',
		defaultMessage: 'Preferred IP stack for Java networking.',
	},
	ipDefault: {
		id: 'app.default-instance.ip.default',
		defaultMessage: 'Default',
	},
	ipPreferV4: {
		id: 'app.default-instance.ip.prefer-v4',
		defaultMessage: 'Prefer IPv4',
	},
	ipPreferV6: {
		id: 'app.default-instance.ip.prefer-v6',
		defaultMessage: 'Prefer IPv6',
	},
	windowTitleTitle: {
		id: 'app.default-instance.window-title.title',
		defaultMessage: 'Game window title',
	},
	windowTitlePlaceholder: {
		id: 'app.default-instance.window-title.placeholder',
		defaultMessage: 'e.g. {name} | Player: {user}',
	},
	windowTitleHint: {
		id: 'app.default-instance.window-title.hint',
		defaultMessage: 'Supports: {user} (username), {name} (instance name), {version} (game version)',
	},
	customInfoTitle: {
		id: 'app.default-instance.custom-info.title',
		defaultMessage: 'Custom info',
	},
	customInfoPlaceholder: {
		id: 'app.default-instance.custom-info.placeholder',
		defaultMessage: 'e.g. MyLauncher',
	},
	customInfoHint: {
		id: 'app.default-instance.custom-info.hint',
		defaultMessage: "Displayed in the game's bottom-left corner and F3 debug screen.",
	},
	javaArgumentsTitle: {
		id: 'app.default-instance.java-arguments.title',
		defaultMessage: 'Java arguments',
	},
	resetToDefault: {
		id: 'app.default-instance.reset-to-default',
		defaultMessage: 'Reset to default',
	},
	javaArgumentsPlaceholder: {
		id: 'app.default-instance.java-arguments.placeholder',
		defaultMessage: 'Enter java arguments...',
	},
	envVariablesTitle: {
		id: 'app.default-instance.env-variables.title',
		defaultMessage: 'Environmental variables',
	},
	envVariablesPlaceholder: {
		id: 'app.default-instance.env-variables.placeholder',
		defaultMessage: 'Enter environmental variables...',
	},
	hooksTitle: {
		id: 'app.default-instance.hooks.title',
		defaultMessage: 'Hooks',
	},
	preLaunchHookTitle: {
		id: 'app.default-instance.pre-launch-hook.title',
		defaultMessage: 'Pre-launch hook',
	},
	preLaunchHookPlaceholder: {
		id: 'app.default-instance.pre-launch-hook.placeholder',
		defaultMessage: 'Enter pre-launch command...',
	},
	preLaunchHookDescription: {
		id: 'app.default-instance.pre-launch-hook.description',
		defaultMessage: 'Ran before the instance is launched.',
	},
	preLaunchWait: {
		id: 'app.default-instance.pre-launch-wait',
		defaultMessage: 'Wait for command to finish before launching',
	},
	wrapperHookTitle: {
		id: 'app.default-instance.wrapper-hook.title',
		defaultMessage: 'Wrapper hook',
	},
	wrapperHookPlaceholder: {
		id: 'app.default-instance.wrapper-hook.placeholder',
		defaultMessage: 'Enter wrapper command...',
	},
	wrapperHookDescription: {
		id: 'app.default-instance.wrapper-hook.description',
		defaultMessage: 'Wrapper command for launching Minecraft.',
	},
	postExitHookTitle: {
		id: 'app.default-instance.post-exit-hook.title',
		defaultMessage: 'Post-exit hook',
	},
	postExitHookPlaceholder: {
		id: 'app.default-instance.post-exit-hook.placeholder',
		defaultMessage: 'Enter post-exit command...',
	},
	postExitHookDescription: {
		id: 'app.default-instance.post-exit-hook.description',
		defaultMessage: 'Ran after the game closes.',
	},
	advancedLaunchOptionsTitle: {
		id: 'app.default-instance.advanced-launch-options.title',
		defaultMessage: 'Advanced launch options',
	},
	rendererTitle: {
		id: 'app.default-instance.renderer.title',
		defaultMessage: 'Renderer',
	},
	rendererDescription: {
		id: 'app.default-instance.renderer.description',
		defaultMessage: 'Override the OpenGL renderer. May cause instability.',
	},
	rendererDefault: {
		id: 'app.default-instance.renderer.default',
		defaultMessage: 'Default',
	},
	rendererLlvmpipe: {
		id: 'app.default-instance.renderer.llvmpipe',
		defaultMessage: 'llvmpipe (software)',
	},
	rendererD3d12: {
		id: 'app.default-instance.renderer.d3d12',
		defaultMessage: 'DirectX 12',
	},
	rendererZink: {
		id: 'app.default-instance.renderer.zink',
		defaultMessage: 'Vulkan (Zink)',
	},
	extraGameArgsTitle: {
		id: 'app.default-instance.extra-game-args.title',
		defaultMessage: 'Extra game arguments',
	},
	extraGameArgsPlaceholder: {
		id: 'app.default-instance.extra-game-args.placeholder',
		defaultMessage: 'e.g. --demo',
	},
	extraGameArgsHint: {
		id: 'app.default-instance.extra-game-args.hint',
		defaultMessage: 'Appended to the end of the Minecraft launch arguments.',
	},
	highPerfGpuTitle: {
		id: 'app.default-instance.high-perf-gpu.title',
		defaultMessage: 'High-performance GPU',
	},
	highPerfGpuDescription: {
		id: 'app.default-instance.high-perf-gpu.description',
		defaultMessage: 'Request the high-performance GPU for the game process.',
	},
	useJavaExeTitle: {
		id: 'app.default-instance.use-java-exe.title',
		defaultMessage: 'Use java.exe',
	},
	useJavaExeDescription: {
		id: 'app.default-instance.use-java-exe.description',
		defaultMessage:
			'Use java.exe instead of javaw.exe. Provides a console window for debugging.',
	},
	compatibilityTitle: {
		id: 'app.default-instance.compatibility.title',
		defaultMessage: 'Compatibility',
	},
	disableJlwTitle: {
		id: 'app.default-instance.disable-jlw.title',
		defaultMessage: 'Disable Java Launch Wrapper',
	},
	disableJlwDescription: {
		id: 'app.default-instance.disable-jlw.description',
		defaultMessage: 'Disable the theseus.jar launch wrapper. May break some features.',
	},
	disableLfTitle: {
		id: 'app.default-instance.disable-lf.title',
		defaultMessage: 'Disable LegacyFix',
	},
	disableLfDescription: {
		id: 'app.default-instance.disable-lf.description',
		defaultMessage: 'Disable compatibility fixes for old Minecraft versions.',
	},
	disableLwjglTitle: {
		id: 'app.default-instance.disable-lwjgl.title',
		defaultMessage: 'Disable LWJGL Unsafe Agent',
	},
	disableLwjglDescription: {
		id: 'app.default-instance.disable-lwjgl.description',
		defaultMessage: 'Disable the LWJGL unsafe agent that patches FFM API performance issues.',
	},
})

const fetchSettings = await get()
fetchSettings.launchArgs = fetchSettings.extra_launch_args.join(' ')
fetchSettings.envVars = fetchSettings.custom_env_vars
	.map((x) => x.join('='))
	.join(' ')
fetchSettings.gameArgs = fetchSettings.extra_game_args.join(' ')

const settings = ref(fetchSettings)

const { maxMemory, snapPoints } = (await useMemorySlider().catch(handleError)) as unknown as {
	maxMemory: number
	snapPoints: number[]
}

const DEFAULT_JVM_ARGS =
	'-XX:+UseG1GC -XX:-UseAdaptiveSizePolicy -XX:-OmitStackTraceInFastThrow -Djdk.lang.Process.allowAmbiguousCommands=true -Dfml.ignoreInvalidMinecraftCertificates=True -Dfml.ignorePatchDiscrepancies=True -Dlog4j2.formatMsgNoLookups=true'

const showJvmReset = computed(
	() => settings.value.launchArgs.trim() !== DEFAULT_JVM_ARGS,
)

function resetJvmArgs() {
	settings.value.launchArgs = DEFAULT_JVM_ARGS
}

const systemMemoryMib = ref(0)
try {
	systemMemoryMib.value = Math.floor((await get_max_memory()) / 1024)
} catch {
	systemMemoryMib.value = maxMemory
}

const memoryWarning = computed(() => {
	if (systemMemoryMib.value === 0) return false
	return settings.value.memory.maximum > systemMemoryMib.value * 0.75
})

const isAutoMemory = computed(
	() => settings.value.memory_allocation_mode === 0,
)

watch(
	settings,
	async () => {
		const setSettings = JSON.parse(JSON.stringify(settings.value))

		setSettings.extra_launch_args = setSettings.launchArgs
			.trim()
			.split(/\s+/)
			.filter(Boolean)
		setSettings.custom_env_vars = setSettings.envVars
			.trim()
			.split(/\s+/)
			.filter(Boolean)
			.map((x) => x.split('=').filter(Boolean))
		setSettings.extra_game_args = setSettings.gameArgs
			.trim()
			.split(/\s+/)
			.filter(Boolean)

		if (!setSettings.hooks.pre_launch) {
			setSettings.hooks.pre_launch = null
		}
		if (!setSettings.hooks.wrapper) {
			setSettings.hooks.wrapper = null
		}
		if (!setSettings.hooks.post_exit) {
			setSettings.hooks.post_exit = null
		}

		if (!setSettings.custom_dir) {
			setSettings.custom_dir = null
		}

		await set(setSettings)
	},
	{ deep: true },
)
</script>

<template>
	<div class="flex flex-col gap-6">
		<!-- Display Options -->
		<div class="flex flex-col gap-6">
			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h3 class="m-0 text-lg font-semibold text-contrast">{{ formatMessage(messages.fullscreenTitle) }}</h3>
					<p class="m-0 leading-tight">
						{{ formatMessage(messages.fullscreenDescription) }}
					</p>
				</div>
				<Toggle id="fullscreen" v-model="settings.force_fullscreen" />
			</div>

			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h3 class="m-0 text-lg font-semibold text-contrast">{{ formatMessage(messages.widthTitle) }}</h3>
					<p class="m-0 leading-tight">{{ formatMessage(messages.widthDescription) }}</p>
				</div>
				<StyledInput
					id="width"
					v-model="settings.game_resolution[0]"
					:disabled="settings.force_fullscreen"
					autocomplete="off"
					type="number"
					:placeholder="formatMessage(messages.widthPlaceholder)"
				/>
			</div>

			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h3 class="m-0 text-lg font-semibold text-contrast">{{ formatMessage(messages.heightTitle) }}</h3>
					<p class="m-0 leading-tight">{{ formatMessage(messages.heightDescription) }}</p>
				</div>
				<StyledInput
					id="height"
					v-model="settings.game_resolution[1]"
					:disabled="settings.force_fullscreen"
					autocomplete="off"
					type="number"
					:placeholder="formatMessage(messages.heightPlaceholder)"
				/>
			</div>
		</div>

		<hr class="my-6 bg-button-border border-none h-[1px]" />

		<!-- Memory -->
		<div class="flex flex-col gap-4">
			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h3 class="m-0 text-lg font-semibold text-contrast">{{ formatMessage(messages.memoryAllocationTitle) }}</h3>
					<p class="m-0 leading-tight">
						{{ formatMessage(messages.memoryAllocationDescription) }}
					</p>
				</div>
				<Combobox
					id="memory-mode"
					:model-value="String(settings.memory_allocation_mode)"
					name="Memory allocation mode"
					class="max-w-40"
					:options="[
						{ value: '0', label: formatMessage(messages.memoryModeAuto) },
						{ value: '1', label: formatMessage(messages.memoryModeCustom) },
					]"
					@update:model-value="(v: string) => settings.memory_allocation_mode = Number(v)"
				/>
			</div>

			<div v-if="!isAutoMemory" class="flex flex-col gap-2.5">
				<div class="flex items-center justify-between">
					<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.memoryAllocated) }}</h4>
					<span class="text-sm text-secondary">{{ settings.memory.maximum }} MB</span>
				</div>
				<Slider
					id="max-memory"
					v-model="settings.memory.maximum"
					:min="512"
					:max="maxMemory"
					:step="64"
					:snap-points="snapPoints"
					:snap-range="512"
					unit="MB"
				/>
				<div
					v-if="memoryWarning"
					class="rounded-lg bg-yellow-bg px-3 py-2 text-sm text-yellow"
				>
					{{ formatMessage(messages.memoryWarning) }}
				</div>
			</div>
			<div v-else class="rounded-lg bg-bg px-3 py-2 text-sm text-secondary">
				{{ formatMessage(messages.memoryAutoHint) }}
			</div>
		</div>

		<hr class="my-6 bg-button-border border-none h-[1px]" />

		<!-- Launch Options -->
		<div class="flex flex-col gap-6">
			<h3 class="m-0 text-lg font-semibold text-contrast">{{ formatMessage(messages.launchOptionsTitle) }}</h3>

			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.processPriorityTitle) }}</h4>
					<p class="m-0 leading-tight">{{ formatMessage(messages.processPriorityDescription) }}</p>
				</div>
				<Combobox
					id="process-priority"
					:model-value="String(settings.process_priority)"
					name="Process priority"
					class="max-w-40"
					:options="[
						{ value: '1', label: formatMessage(messages.priorityNormal) },
						{ value: '0', label: formatMessage(messages.priorityAboveNormal) },
						{ value: '3', label: formatMessage(messages.priorityHigh) },
						{ value: '2', label: formatMessage(messages.priorityBelowNormal) },
						{ value: '4', label: formatMessage(messages.priorityRealtime) },
					]"
					@update:model-value="(v: string) => settings.process_priority = Number(v)"
				/>
			</div>

			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.ipProtocolTitle) }}</h4>
					<p class="m-0 leading-tight">{{ formatMessage(messages.ipProtocolDescription) }}</p>
				</div>
				<Combobox
					id="ip-stack"
					:model-value="String(settings.preferred_ip_stack)"
					name="IP stack"
					class="max-w-40"
					:options="[
						{ value: '1', label: formatMessage(messages.ipDefault) },
						{ value: '0', label: formatMessage(messages.ipPreferV4) },
						{ value: '2', label: formatMessage(messages.ipPreferV6) },
					]"
					@update:model-value="(v: string) => settings.preferred_ip_stack = Number(v)"
				/>
			</div>

			<div class="flex flex-col gap-2.5">
				<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.windowTitleTitle) }}</h4>
				<StyledInput
					id="window-title"
					v-model="settings.window_title"
					autocomplete="off"
					type="text"
					:placeholder="formatMessage(messages.windowTitlePlaceholder)"
					wrapper-class="w-full"
				/>
				<p class="m-0 leading-tight">
					{{ formatMessage(messages.windowTitleHint) }}
				</p>
			</div>

			<div class="flex flex-col gap-2.5">
				<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.customInfoTitle) }}</h4>
				<StyledInput
					id="custom-info"
					v-model="settings.custom_info"
					autocomplete="off"
					type="text"
					:placeholder="formatMessage(messages.customInfoPlaceholder)"
					wrapper-class="w-full"
				/>
				<p class="m-0 leading-tight">
					{{ formatMessage(messages.customInfoHint) }}
				</p>
			</div>
		</div>

		<hr class="my-6 bg-button-border border-none h-[1px]" />

		<!-- JVM Arguments -->
		<div class="flex flex-col gap-4">
			<div class="flex items-center justify-between">
				<h3 class="m-0 text-lg font-semibold text-contrast">{{ formatMessage(messages.javaArgumentsTitle) }}</h3>
				<button
					v-if="showJvmReset"
					class="text-sm text-link hover:underline"
					@click="resetJvmArgs"
				>
					{{ formatMessage(messages.resetToDefault) }}
				</button>
			</div>
			<StyledInput
				id="java-args"
				v-model="settings.launchArgs"
				autocomplete="off"
				type="text"
				:placeholder="formatMessage(messages.javaArgumentsPlaceholder)"
				wrapper-class="w-full"
			/>

			<div class="flex flex-col gap-2.5">
				<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.envVariablesTitle) }}</h4>
				<StyledInput
					id="env-vars"
					v-model="settings.envVars"
					autocomplete="off"
					type="text"
					:placeholder="formatMessage(messages.envVariablesPlaceholder)"
					wrapper-class="w-full"
				/>
			</div>
		</div>

		<hr class="my-6 bg-button-border border-none h-[1px]" />

		<!-- Hooks -->
		<div class="flex flex-col gap-6">
			<h3 class="m-0 text-lg font-semibold text-contrast">{{ formatMessage(messages.hooksTitle) }}</h3>

			<div class="flex flex-col gap-2.5">
				<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.preLaunchHookTitle) }}</h4>
				<StyledInput
					id="pre-launch"
					v-model="settings.hooks.pre_launch"
					autocomplete="off"
					type="text"
					:placeholder="formatMessage(messages.preLaunchHookPlaceholder)"
					wrapper-class="w-full"
				/>
				<p class="m-0 leading-tight">{{ formatMessage(messages.preLaunchHookDescription) }}</p>
				<div v-if="settings.hooks.pre_launch" class="flex items-center gap-2 mt-1">
					<Toggle id="pre-launch-wait" v-model="settings.pre_launch_wait" />
					<label for="pre-launch-wait" class="text-sm text-secondary">
						{{ formatMessage(messages.preLaunchWait) }}
					</label>
				</div>
			</div>

			<div class="flex flex-col gap-2.5">
				<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.wrapperHookTitle) }}</h4>
				<StyledInput
					id="wrapper"
					v-model="settings.hooks.wrapper"
					autocomplete="off"
					type="text"
					:placeholder="formatMessage(messages.wrapperHookPlaceholder)"
					wrapper-class="w-full"
				/>
				<p class="m-0 leading-tight">{{ formatMessage(messages.wrapperHookDescription) }}</p>
			</div>

			<div class="flex flex-col gap-2.5">
				<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.postExitHookTitle) }}</h4>
				<StyledInput
					id="post-exit"
					v-model="settings.hooks.post_exit"
					autocomplete="off"
					type="text"
					:placeholder="formatMessage(messages.postExitHookPlaceholder)"
					wrapper-class="w-full"
				/>
				<p class="m-0 leading-tight">{{ formatMessage(messages.postExitHookDescription) }}</p>
			</div>
		</div>

		<hr class="my-6 bg-button-border border-none h-[1px]" />

		<!-- Advanced Launch Options -->
		<div class="flex flex-col gap-6">
			<h3 class="m-0 text-lg font-semibold text-contrast">{{ formatMessage(messages.advancedLaunchOptionsTitle) }}</h3>

			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.rendererTitle) }}</h4>
					<p class="m-0 leading-tight">
						{{ formatMessage(messages.rendererDescription) }}
					</p>
				</div>
				<Combobox
					id="renderer"
					:model-value="String(settings.renderer)"
					name="Renderer"
					class="max-w-40"
					:options="[
						{ value: '0', label: formatMessage(messages.rendererDefault) },
						{ value: '1', label: formatMessage(messages.rendererLlvmpipe) },
						{ value: '2', label: formatMessage(messages.rendererD3d12) },
						{ value: '3', label: formatMessage(messages.rendererZink) },
					]"
					@update:model-value="(v: string) => settings.renderer = Number(v)"
				/>
			</div>

			<div class="flex flex-col gap-2.5">
				<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.extraGameArgsTitle) }}</h4>
				<StyledInput
					id="game-args"
					v-model="settings.gameArgs"
					autocomplete="off"
					type="text"
					:placeholder="formatMessage(messages.extraGameArgsPlaceholder)"
					wrapper-class="w-full"
				/>
				<p class="m-0 leading-tight">{{ formatMessage(messages.extraGameArgsHint) }}</p>
			</div>

			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.highPerfGpuTitle) }}</h4>
					<p class="m-0 leading-tight">
						{{ formatMessage(messages.highPerfGpuDescription) }}
					</p>
				</div>
				<Toggle id="gpu-pref" v-model="settings.set_gpu_preference" />
			</div>

			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.useJavaExeTitle) }}</h4>
					<p class="m-0 leading-tight">
						{{ formatMessage(messages.useJavaExeDescription) }}
					</p>
				</div>
				<Toggle id="use-java-exe" v-model="settings.use_java_exe" />
			</div>
		</div>

		<hr class="my-6 bg-button-border border-none h-[1px]" />

		<!-- Compatibility Toggles -->
		<div class="flex flex-col gap-6">
			<h3 class="m-0 text-lg font-semibold text-contrast">{{ formatMessage(messages.compatibilityTitle) }}</h3>

			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h4 class="m-0 text-sm font-semibold text-contrast">
						{{ formatMessage(messages.disableJlwTitle) }}
					</h4>
					<p class="m-0 leading-tight">
						{{ formatMessage(messages.disableJlwDescription) }}
					</p>
				</div>
				<Toggle
					id="disable-jlw"
					v-model="settings.disable_java_launch_wrapper"
				/>
			</div>

			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h4 class="m-0 text-sm font-semibold text-contrast">{{ formatMessage(messages.disableLfTitle) }}</h4>
					<p class="m-0 leading-tight">
						{{ formatMessage(messages.disableLfDescription) }}
					</p>
				</div>
				<Toggle
					id="disable-lf"
					v-model="settings.disable_legacy_fix"
				/>
			</div>

			<div class="flex items-center justify-between gap-4">
				<div class="flex flex-col gap-1">
					<h4 class="m-0 text-sm font-semibold text-contrast">
						{{ formatMessage(messages.disableLwjglTitle) }}
					</h4>
					<p class="m-0 leading-tight">
						{{ formatMessage(messages.disableLwjglDescription) }}
					</p>
				</div>
				<Toggle
					id="disable-lwjgl"
					v-model="settings.disable_lwjgl_unsafe_agent"
				/>
			</div>
		</div>
	</div>
</template>
