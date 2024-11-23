<template>
  <WidgetPane title="Playlists">
    <div class="column playback-sources">
      <ToolPane class="col-auto">
        <q-btn
          icon="playlist_add"
          label="Add to playlist"
          @click="addToPlaylist()"
          :disable="!isAddButtonActive"
        />
      </ToolPane>
      <div class="row">
        <div
          v-for="playlist in playlistsList"
          :key="playlist.id"
          class="playlist-item row flex-center col-4 q-pa-sm"
          @click="selectPlaylist(playlist.id)"
          :class="{ 'playlist-item-selected': playlist.id === selectedPlaylistId }"
        >
          <q-icon v-if="playlist.added" name="playlist_add_check" class="icon-added-to-playlist q-pr-md" />
          <div>{{ playlist.name }}</div>
        </div>
      </div>
    </div>
  </WidgetPane>
</template>

<script setup>
import { ref, getCurrentInstance, computed } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const playlistsList = ref([])
const selectedPlaylistId = ref(-1)
let musicItemId = -1

async function selectPlaylist (playlistId) {
  selectedPlaylistId.value = playlistId
}

async function addToPlaylist () {
  await aminaApi.sendRequest('lappi.playlists.add_item_to_playlist', {
    playlist_id: selectedPlaylistId.value,
    music_item_id: musicItemId
  })
  selectedPlaylistId.value = -1
}

async function update () {
  const newPlaylistsList = await aminaApi.sendRequest('lappi.playlists.get_playlists')
  const addedPlaylistsList = await aminaApi.sendRequest('lappi.playlists.get_playlists_for_music_item', { music_item_id: musicItemId })

  for (const index in newPlaylistsList) {
    if (addedPlaylistsList.includes(newPlaylistsList[index].id)) {
      newPlaylistsList[index].added = true
    } else {
      newPlaylistsList[index].added = false
    }
  }

  playlistsList.value = newPlaylistsList
}

async function setMusicItem (itemId) {
  musicItemId = itemId
  selectedPlaylistId.value = -1
  await update()
}

const isAddButtonActive = computed(() => {
  if (selectedPlaylistId.value >= 0) {
    return playlistsList.value.find(playlist => playlist.id === selectedPlaylistId.value).added === false
  }
  return false
})

defineExpose({
  update,
  setMusicItem
})
</script>

<style lang="sass" scoped>
.playlist-item
  text-align: center
  cursor: pointer
  &:hover
    background-color: $amina-primary-color

.playlist-item-selected
  background-color: $amina-primary-color !important
  border: 1px solid $amina-positive

.icon-added-to-playlist
  color: $amina-positive

</style>
