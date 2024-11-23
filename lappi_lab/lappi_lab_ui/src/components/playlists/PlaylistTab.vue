<template>
  <div class="column q-gutter-md">
    <div class="editor-header text-subtitle1 col q-pa-lg items-center q-gutter-sm">
      <div class="text-weight-medium">{{ playlistName }}</div>
    </div>
    <WidgetPane title="Music items" class="col">
      <q-table
        flat dense
        :rows="playlistItems"
        :columns="playlistItemsColumns"
        row-key="music_item_id"
        :rows-per-page-options="[0]"
      />
    </WidgetPane>
    <PlaylistSummary ref="playlistSummary" />
  </div>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import PlaylistSummary from 'src/components/playlists/PlaylistSummary.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const playlistSummary = ref(null)

const playlistItemsColumns = [
  {
    name: 'music_item_id',
    label: '#',
    field: 'music_item_id',
    align: 'left'
  },
  {
    name: 'title',
    label: 'Title',
    field: 'title',
    align: 'left'
  },
  {
    name: 'album',
    label: 'Album',
    field: 'album',
    align: 'center'
  },
  {
    name: 'artist',
    label: 'Artist',
    field: 'artist',
    align: 'center'
  }
]

const playlistItems = ref([])
const playlistName = ref(null)

async function updatePlaylist (newPlaylistId) {
  const playlistDescription = await aminaApi.sendRequest('lappi.playlists.get_playlist_description', { playlist_id: newPlaylistId })
  playlistName.value = playlistDescription.name
  playlistSummary.value.update(newPlaylistId)

  const newPlaylistItems = await aminaApi.sendRequest('lappi.playlists.get_playlist_items', { playlist_id: newPlaylistId })
  playlistItems.value = newPlaylistItems
}

defineExpose({
  updatePlaylist
})
</script>

<style lang="sass" scoped>
.editor-header
  text-align: center
  text-shadow: 0px 0px 16px rgba(255,255,255,0.4)

</style>
