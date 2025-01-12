<template>
  <div class="row q-pa-md q-gutter-md">
    <WidgetPane class="col-3">
      <AbsoluteWrapper class="list-wrapper col">
        <q-virtual-scroll class="list-scroll" :items="playlistsList" v-slot="{ item }">
          <q-item class="list-item" :key="item.id" clickable @click="selectPlaylist(item.id)">
            <q-item-section avatar>
              <q-avatar v-show="item.hasAvatar === true" rounded>
                <img :src="item.pictureUrl">
              </q-avatar>
              <q-icon v-show="item.hasAvatar === false" name="queue_music" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ item.name }}</q-item-label>
              <q-item-label caption>Playlist</q-item-label>
            </q-item-section>
          </q-item>
        </q-virtual-scroll>
      </AbsoluteWrapper>
      <ToolPane class="col-auto">
        <q-btn label="Add Playlist" @click="addNewPlaylist" />
      </ToolPane>
    </WidgetPane>
    <div v-show="selectedPlaylistId < 0" class="empty-playlist-wrapper col"></div>
    <AbsoluteWrapper v-show="selectedPlaylistId >= 0" class="col ">
      <q-scroll-area style="height: 100%; max-width: 100%;">
        <PlaylistTab class="col" ref="playlistTab" />
      </q-scroll-area>
    </AbsoluteWrapper>
  </div>
</template>

<script setup>
import { getCurrentInstance, onMounted, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import AbsoluteWrapper from 'src/amina_ui/components/AbsoluteWrapper.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import PlaylistTab from 'src/components/playlists/PlaylistTab.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const playlistTab = ref(null)

const playlistsList = ref([])
const selectedPlaylistId = ref(-1)

async function addNewPlaylist () {
  await aminaApi.sendRequest('lappi.playlists.create_default_playlist')
}

async function selectPlaylist (newPlaylistId) {
  selectedPlaylistId.value = newPlaylistId
  await playlistTab.value.updatePlaylist(newPlaylistId)
}

async function update () {
  const newPlaylistsList = []

  for (const playlistDesc of await aminaApi.sendRequest('lappi.playlists.get_playlists')) {
    const item = {
      id: playlistDesc.id,
      name: playlistDesc.name
    }

    if ('avatar_picture_id' in playlistDesc) {
      const path = await aminaApi.sendRequest('lappi.collection.pictures.get_picture_path', { picture_id: playlistDesc.avatar_picture_id })
      const pictureUrl = await aminaApi.getFileUrl(path)

      item.hasAvatar = true
      item.pictureUrl = pictureUrl
    } else {
      item.hasAvatar = false
    }

    newPlaylistsList.push(item)
  }

  playlistsList.value = newPlaylistsList
  if (selectedPlaylistId.value >= 0) {
    await playlistTab.value.updatePlaylist(selectedPlaylistId.value)
  }
}

aminaApi.setEventHandler('lappi.collection.OnCollectionUpdated', 'PlaylistPane', (event) => {
  update()
})

onMounted(() => {
  update()
})
</script>

<style lang="sass" scoped>
.empty-playlist-wrapper
  height: 100%
  width: 100%
  background-image: url( '~assets/lappi_pattern.svg' )
  background-position: center
  background-size: 350px
  background-repeat: no-repeat

</style>
