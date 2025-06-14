<template>
  <WidgetPane title="Music file">
    <div class="column">
      <ToolPane class="col-auto">
        <q-btn icon="play_arrow" label="Play" class="play-button" @click="playMusicFile()" />
        <q-btn icon="add" label="Import" class="import-button" />
        <q-btn icon="delete" label="Delete" class="delete-button" @click="deleteMusicFile()" />
      </ToolPane>
      <div v-if="isFileExists" class="row items-center q-pa-md">
        <div class="col-auto q-pr-md">Path:</div>
        <q-input
            dense square filled readonly
            class="col"
            v-model="musicFilePath"
        />
      </div>
      <div v-else class="col text-center q-pa-md">
        No music file
      </div>
    </div>
  </WidgetPane>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

let currentItemId = -1

const isFileExists = ref(false)
const musicFilePath = ref('')

async function updateItem (itemId) {
  currentItemId = itemId
  const musicFileDescOption = await aminaApi.sendRequest('lappi.collection.music_sources.get_music_file', { item_id: currentItemId })
  console.log(musicFileDescOption)
  if (musicFileDescOption) {
    isFileExists.value = true
    const musicFileId = musicFileDescOption.internal_file_id
    const newMusicFilePath = await aminaApi.sendRequest('lappi.collection.internal_files.get_file_path', { file_id: musicFileId })
    musicFilePath.value = newMusicFilePath.path
  } else {
    isFileExists.value = false
    musicFilePath.value = ''
  }
}

async function playMusicFile () {
  if (currentItemId) {
    await aminaApi.sendRequest('lappi.playback.play_item', { item_id: currentItemId })
  }
}

async function deleteMusicFile () {
  await aminaApi.sendRequest('lappi.collection.music_sources.delete_music_file', { item_id: currentItemId })
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>
.play-button
  color: $amina-positive

.import-button
  color: $amina-info

.delete-button
  color: $amina-negative

</style>
