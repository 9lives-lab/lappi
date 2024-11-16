<template>
  <WidgetPane title="Scripts">
    <div class="column">
      <q-linear-progress :query="isInProgress" />
      <div class="row q-pa-md q-gutter-md">
        <q-btn
          v-for="item in scriptsList"
          :key="item"
          :label="item"
          color="primary"
          no-caps
          @click="runScript(item)"
        />
      </div>
    </div>
  </WidgetPane>
</template>

<script setup>
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import { ref, getCurrentInstance } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const scriptsList = ref([])
const isInProgress = ref(false)

let musicItemId = -1

async function update () {
  const newScriptsList = await aminaApi.sendRequest('lappi.scripting_engine.get_scripts_list')
  scriptsList.value = newScriptsList
  isInProgress.value = false
}

async function setMusicItem (itemId) {
  await update()
  musicItemId = itemId
}

async function runScript (scriptName) {
  isInProgress.value = true
  await aminaApi.sendRequest('lappi.scripting_engine.run_for_music_item', { script_name: scriptName, music_item_id: musicItemId })
  isInProgress.value = false
}

defineExpose({
  update,
  setMusicItem
})
</script>
