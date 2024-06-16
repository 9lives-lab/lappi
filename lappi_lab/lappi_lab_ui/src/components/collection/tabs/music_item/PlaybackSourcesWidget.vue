<template>
  <WidgetPane title="Play">
    <div class="column playback-sources">
      <ToolPane class="col-auto">
        <q-btn icon="play_arrow" label="Play" @click="playMusicFile()" />
      </ToolPane>
      <q-list separator>
        <q-item clickable v-ripple v-for="source in sources" :key="source.id">
          <q-item-section side>
            <q-item-label>{{ source.name }}</q-item-label>
            <q-item-label caption>{{ source.description }}</q-item-label>
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
  const externalFiles = await aminaApi.sendRequest('lappi.collection.music.get_external_src_files', { item_id: currentItemId })
  sources.value = externalFiles.map(fileDescription => ({
    id: fileDescription.id,
    name: fileDescription.path,
    description: 'Local file'
  }))
}

async function playMusicFile () {
  if (currentItemId) {
    await aminaApi.sendRequest('lappi.playback.play_item', { item_id: currentItemId })
  }
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>

</style>
