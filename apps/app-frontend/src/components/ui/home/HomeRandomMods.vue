<script setup lang="ts">
import { SparklesIcon } from '@modrinth/assets'
import { ButtonStyled, defineMessages, HeadingLink, injectNotificationManager, useVIntl } from '@modrinth/ui'
import type { SearchResult } from '@modrinth/utils'
import { onMounted, ref } from 'vue'

import LegacyProjectCard from '@/components/ui/LegacyProjectCard.vue'
import { get_search_results } from '@/helpers/cache.js'

const { handleError } = injectNotificationManager()

const { formatMessage } = useVIntl()

const messages = defineMessages({
  title: { id: 'app.home.random-mods.title', defaultMessage: 'Random mod spotlight' },
  reroll: { id: 'app.home.random-mods.reroll', defaultMessage: 'Reroll' },
})

const randomMods = ref<SearchResult[]>([])
const loading = ref(true)

const MAX_MODS = 6

async function fetchRandomMods() {
	loading.value = true
	try {
		// Use random index for daily discovery, mod project type only
		const response = await get_search_results(
			`?facets=[["project_type:mod"]]&limit=${MAX_MODS}&index=random`,
		)
		if (response) {
			randomMods.value = response.result.hits
		} else {
			randomMods.value = []
		}
	} catch (e) {
		handleError(e as Error)
		randomMods.value = []
	} finally {
		loading.value = false
	}
}

function reroll() {
	fetchRandomMods()
}

onMounted(fetchRandomMods)
</script>

<template>
	<div v-if="randomMods.length > 0" class="flex flex-col gap-2">
		<div class="flex items-center justify-between">
			<HeadingLink to="/browse/mod">
				<SparklesIcon class="inline mr-1" />
			{{ formatMessage(messages.title) }}
		</HeadingLink>
			<ButtonStyled type="transparent">
				<button class="!text-sm" @click="reroll">
					<SparklesIcon class="size-4" />
				{{ formatMessage(messages.reroll) }}
			</button>
			</ButtonStyled>
		</div>
		<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
			<LegacyProjectCard
				v-for="project in randomMods"
				:key="project?.project_id"
				:project="project"
				class="item"
			/>
		</div>
	</div>
</template>

<style scoped lang="scss">
.item {
	width: 100%;
	max-width: 100%;
}
</style>
