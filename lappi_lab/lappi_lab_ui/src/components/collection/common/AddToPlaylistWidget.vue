<template>
  <WidgetPane title="Playlists">
    <div class="column playback-sources">
      <div class="row q-pa-md q-gutter-md">
        <q-btn
          no-caps
          v-for="playlist in playlistsList"
          :key="playlist.id"
          :color="playlist.added ? 'primary' : 'blue-grey'"
          :icon="playlist.added ? 'check' : 'add'"
          :label="playlist.name"
          @click="addOrRemoveFromPlaylist(playlist)"
        />
      </div>
    </div>
  </WidgetPane>
</template>

<script setup>
import { ref, getCurrentInstance } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const playlistsList = ref([])
let musicItemId = -1

async function addOrRemoveFromPlaylist (playlist) {
  if (playlist.added) {
    await aminaApi.sendRequest('lappi.playlists.delete_item_from_playlist', {
      playlist_id: playlist.id,
      music_item_id: musicItemId
    })
  } else {
    await aminaApi.sendRequest('lappi.playlists.add_item_to_playlist', {
      playlist_id: playlist.id,
      music_item_id: musicItemId
    })
  }
  await update()
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
  await update()
}

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
    background-color: $primary

.playlist-item-selected
  background-color: $amina-primary-color !important
  border: 1px solid $primary

.icon-added-to-playlist
  color: $amina-positive

</style>
