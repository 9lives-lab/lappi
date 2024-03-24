<template>
  <div class="column pictures-editor">
    <PicturesViewer ref="picturesViewer" />
    <ToolPane class="col">
      <q-btn class="col-auto" icon="add" label="Add" @click="addPicture" />
      <q-input borderless dense class="col" v-model="addPath" />
    </ToolPane>
  </div>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import PicturesViewer from 'src/components/collection/pictures/PicturesViewer.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const picturesViewer = ref(null)
const addPath = ref('')
let config = null

async function addPicture () {
  const path = addPath.value
  const pictureId = await aminaApi.sendRequest('lappi.collection.pictures.copy_to_collection_by_path', { file_path: path })
  addPath.value = ''
  await aminaApi.sendRequest(config.addPictureRpcKey, { picture_id: pictureId, item_id: config.id })
  picturesViewer.value.update(config)
}

async function update (newConfig) {
  config = newConfig
  picturesViewer.value.update(newConfig)
}

defineExpose({
  update
})
</script>

<style lang="sass" scoped>

</style>
