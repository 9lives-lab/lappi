<template>
  <WidgetPane title="Play">
    <div class="column playback-sources">
      <ToolPane class="col-auto">
        <q-btn icon="play_arrow" label="Play" @click="playMusicFile()" />
        <q-btn icon="add" label="Add local file" @click="addLocalSourceFile()" />
        <q-btn icon="add" label="Add URL" @click="addUrlSourceFile()" />
      </ToolPane>
      <q-list separator>
        <q-item clickable v-ripple v-for="source in sources" :key="source.id">
          <q-item-section side>
            <q-item-label>{{ source.description }}</q-item-label>
          </q-item-section>
          <q-item-section>
            <q-input
              dense square filled
              v-model="source.name"
              @update:model-value="newValue => setSourcePath(newValue, source)"
            />
          </q-item-section>
          <q-item-section side>
            <q-btn class="delete-button" icon="delete" @click="deleteSourceFile(source)" />
          </q-item-section>
        </q-item>
      </q-list>
    </div>
  </WidgetPane>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

let currentItemId = -1
const sources = ref([])

async function updateItem (itemId) {
  currentItemId = itemId
  const externalFiles = await aminaApi.sendRequest('lappi.collection.music.get_source_files', { item_id: currentItemId })
  sources.value = externalFiles.map(fileDescription => ({
    id: fileDescription.id,
    name: fileDescription.path,
    description: fileDescription.source_type
  }))
}

async function playMusicFile () {
  if (currentItemId) {
    await aminaApi.sendRequest('lappi.playback.play_item', { item_id: currentItemId })
  }
}

async function addUrlSourceFile () {
  await aminaApi.sendRequest('lappi.collection.music.add_source_file', { item_id: currentItemId, source_type: 'Url', path: '' })
}

async function addLocalSourceFile () {
  await aminaApi.sendRequest('lappi.collection.music.add_source_file', { item_id: currentItemId, source_type: 'LocalFile', path: '' })
}

async function setSourcePath (newValue, source) {
  await aminaApi.sendRequest('lappi.collection.music.set_source_file_path', { source_id: source.id, path: newValue })
}

async function deleteSourceFile (source) {
  await aminaApi.sendRequest('lappi.collection.music.delete_source_file', { source_id: source.id })
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>
.delete-button
  color: $amina-negative

</style>
