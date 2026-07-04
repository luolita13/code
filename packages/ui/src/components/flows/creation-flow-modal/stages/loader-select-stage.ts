import { LeftArrowIcon, PlusIcon, RightArrowIcon } from '@modrinth/assets'
import { markRaw } from 'vue'

import { commonMessages } from '#ui/utils/common-messages'

import type { StageConfigInput } from '../../../base'
import LoaderSelectStage from '../components/LoaderSelectStage.vue'
import {
	type CreationFlowContextValue,
	creationFlowMessages,
	flowTypeHeadingMessages,
} from '../creation-flow-context'

function isForwardBlocked(ctx: CreationFlowContextValue): boolean {
	if (!ctx.selectedLoader.value) return true
	// Non-vanilla loaders must have a loader version selected
	if (ctx.selectedLoader.value !== 'vanilla' && !ctx.selectedLoaderVersion.value) return true
	return false
}

export const stageConfig: StageConfigInput<CreationFlowContextValue> = {
	id: 'loader-select',
	title: (ctx) => ctx.formatMessage(flowTypeHeadingMessages[ctx.flowType]),
	stageContent: markRaw(LoaderSelectStage),
	// Only show for instance flow's custom setup; skip for modpack/import/vanilla
	skip: (ctx) =>
		ctx.flowType !== 'instance' ||
		ctx.setupType.value !== 'custom' ||
		ctx.isImportMode.value,
	cannotNavigateForward: isForwardBlocked,
	leftButtonConfig: (ctx) => ({
		label: ctx.formatMessage(commonMessages.backButton),
		icon: LeftArrowIcon,
		onClick: () => ctx.modal.value?.setStage('game-version-select'),
	}),
	rightButtonConfig: (ctx) => {
		const isInstance = ctx.flowType === 'instance'
		const disabled = isForwardBlocked(ctx)

		if (isInstance) {
			return {
				label: ctx.formatMessage(creationFlowMessages.createInstanceButton),
				icon: PlusIcon,
				iconPosition: 'before' as const,
				color: 'brand' as const,
				disabled: disabled || ctx.finishDisabled.value,
				loading: ctx.loading.value,
				tooltip: ctx.finishDisabled.value ? ctx.finishDisabledTooltip.value : undefined,
				onClick: () => ctx.finish(),
			}
		}

		return {
			label: ctx.formatMessage(commonMessages.continueButton),
			icon: RightArrowIcon,
			iconPosition: 'after' as const,
			disabled,
			onClick: () => ctx.modal.value?.nextStage(),
		}
	},
	maxWidth: '600px',
}
