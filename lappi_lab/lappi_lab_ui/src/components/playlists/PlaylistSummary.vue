<template>
  <WidgetPane title="Summary">
    <div class="row">
      <div class="column col q-pa-md q-gutter-md">
        <div class="row items-center">
          <div class="col-auto q-pr-md">Playlist name:</div>
          <q-input
            dense square filled
            class="col"
            v-model="playlistName"
            @update:model-value="setName"
          />
        </div>
      </div>
      <div class="column col q-pa-md q-gutter-md">
        <div class="row items-center">
          <div class="col-auto q-pr-md">Playlist ID:</div>
          <q-input
            dense square filled readonly
            class="col"
            v-model="playlistId"
          />
        </div>
      </div>
    </div>
  </WidgetPane>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const playlistId = ref(0)
const playlistName = ref('')

async function update (newPlaylistId) {
  playlistId.value = newPlaylistId
  const playlistDescription = await aminaApi.sendRequest('lappi.playlists.get_playlist_description', { playlist_id: newPlaylistId })
  playlistName.value = playlistDescription.name
}

async function setName (newName) {
  await aminaApi.sendRequest('lappi.playlists.set_playlist_name', { playlist_id: playlistId.value, name: newName })
}

defineExpose({
  update
})
</script>

<style lang="sass" scoped>

</style>
