<script setup lang="ts">
import { UpdatedIcon } from '@modrinth/assets'
import { Avatar, ButtonStyled, defineMessages, HeadingLink, injectNotificationManager, useVIntl } from '@modrinth/ui'
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import type { LinkedModpackInfo } from '@/helpers/instance'
import { get_linked_modpack_info, list } from '@/helpers/instance'
import type { GameInstance } from '@/helpers/types'

const { handleError } = injectNotificationManager()
const router = useRouter()

const { formatMessage } = useVIntl()

const messages = defineMessages({
  title: { id: 'app.home.update-reminders.title', defaultMessage: 'Updates available' },
  newVersion: { id: 'app.home.update-reminders.new-version', defaultMessage: 'New version' },
  update: { id: 'app.home.update-reminders.update', defaultMessage: 'Update' },
})

const instances = ref<GameInstance[]>([])
const loading = ref(true)

interface UpdateEntry {
	instance: GameInstance
	info: LinkedModpackInfo
}

const updatesAvailable = ref<UpdateEntry[]>([])

async function fetchUpdates() {
	loading.value = true
	try {
		const all = await list().catch(handleError)
		instances.value = (all ?? []).filter(
			(x) => x.install_stage === 'installed' && x.link?.project_id && x.link?.version_id,
		)

		const results: UpdateEntry[] = []
		await Promise.all(
			instances.value.map(async (inst) => {
				try {
					const info = await get_linked_modpack_info(inst.id, 'must_revalidate')
					if (info?.has_update) {
						results.push({ instance: inst, info })
					}
				} catch {
					// Skip on error
				}
			}),
		)
		updatesAvailable.value = results
	} finally {
		loading.value = false
	}
}

const hasUpdates = computed(() => updatesAvailable.value.length > 0)

function viewInstance(id: string) {
	router.push(`/instance/${encodeURIComponent(id)}`)
}

onMounted(fetchUpdates)
</script>

<template>
	<div v-if="hasUpdates" class="flex flex-col gap-2">
		<HeadingLink to="/library">
			<UpdatedIcon class="inline mr-1" />
			{{ formatMessage(messages.title) }}
		</HeadingLink>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
			<div
				v-for="entry in updatesAvailable"
				:key="entry.instance.id"
				class="update-card flex items-center gap-3 p-3 rounded-xl bg-surface-2 hover:bg-surface-3 transition-colors cursor-pointer"
				@click="viewInstance(entry.instance.id)"
			>
				<Avatar
					:src="entry.info.project?.icon_url"
					:alt="entry.instance.name"
					size="2.5rem"
				/>
				<div class="flex-1 min-w-0">
					<p class="m-0 text-sm font-semibold text-contrast truncate">
						{{ entry.info.project?.title ?? entry.instance.name }}
					</p>
					<p class="m-0 text-xs text-secondary truncate">
						{{ entry.info.update_version?.name ?? formatMessage(messages.newVersion) }}
					</p>
				</div>
				<ButtonStyled type="brand">
				<button class="!px-3 !py-1.5 !text-xs">{{ formatMessage(messages.update) }}</button>
			</ButtonStyled>
			</div>
		</div>
	</div>
</template>

<style scoped lang="scss">
.update-card {
	border: 1px solid var(--color-bg);
}
</style>
