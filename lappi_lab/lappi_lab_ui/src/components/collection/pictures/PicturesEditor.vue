<template>
  <div class="column pictures-editor">
    <ToolPane class="col">
      <q-btn class="col-auto" icon="add" label="Add" @click="addPicture" />
      <q-input borderless dense class="col" v-model="addPath" />
    </ToolPane>
    <PicturesViewer ref="picturesViewer" />
  </div>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import PicturesViewer from 'src/components/collection/pictures/PicturesViewer.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const picturesViewer = ref(null)
const addPath = ref('')
let folderId = 0

async function addPicture () {
  const path = addPath.value
  await aminaApi.sendRequest('lappi.collection.pictures.copy_to_collection_by_path', { file_path: path, folder_id: folderId })
  addPath.value = ''
  picturesViewer.value.update(folderId)
}

async function update (newFolderId) {
  folderId = newFolderId
  picturesViewer.value.update(newFolderId)
}

defineExpose({
  update
})
</script>

<style lang="sass" scoped>

</style>
