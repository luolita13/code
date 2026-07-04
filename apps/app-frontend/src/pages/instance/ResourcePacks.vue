<template>
	<ResourcePackManager :instance="instance" :folder="folder" :project-type="projectType" />
</template>

<script setup lang="ts">
import type ContextMenu from '@/components/ui/ContextMenu.vue'
import ResourcePackManager from '@/components/ui/ResourcePackManager.vue'
import type { GameInstance } from '@/helpers/types'

const props = defineProps<{
	instance: GameInstance
	options: InstanceType<typeof ContextMenu> | null
	offline: boolean
	playing: boolean
	installed: boolean
	isServerInstance?: boolean
}>()

const folder = 'resourcepacks'
const projectType = 'resourcepack'
</script>
