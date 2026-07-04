import { LeftArrowIcon, RightArrowIcon } from '@modrinth/assets'
import { markRaw } from 'vue'

import { commonMessages } from '#ui/utils/common-messages'

import type { StageConfigInput } from '../../../base'
import GameVersionSelectStage from '../components/GameVersionSelectStage.vue'
import {
	type CreationFlowContextValue,
	creationFlowMessages,
	flowTypeHeadingMessages,
} from '../creation-flow-context'

function isForwardBlocked(ctx: CreationFlowContextValue): boolean {
	return !ctx.selectedGameVersion.value
}

export const stageConfig: StageConfigInput<CreationFlowContextValue> = {
	id: 'game-version-select',
	title: (ctx) => ctx.formatMessage(flowTypeHeadingMessages[ctx.flowType]),
	stageContent: markRaw(GameVersionSelectStage),
	// Only show for instance flow's custom setup; skip for modpack/import/vanilla
	skip: (ctx) =>
		ctx.flowType !== 'instance' ||
		ctx.setupType.value !== 'custom' ||
		ctx.isImportMode.value,
	cannotNavigateForward: isForwardBlocked,
	leftButtonConfig: (ctx) => ({
		label: ctx.formatMessage(commonMessages.backButton),
		icon: LeftArrowIcon,
		onClick: () => ctx.modal.value?.setStage('setup-type'),
	}),
	rightButtonConfig: (ctx) => ({
		label: ctx.formatMessage(commonMessages.continueButton),
		icon: RightArrowIcon,
		iconPosition: 'after' as const,
		disabled: isForwardBlocked(ctx),
		onClick: () => ctx.modal.value?.setStage('loader-select'),
	}),
	maxWidth: '600px',
}
