<template>
  <WidgetPane title="Pictures">
    <div class="column pictures-editor">
      <ToolPane class="col">
        <q-file
          multiple
          dense
          outlined
          label="Add pictures"
          v-model="files"
          @update:model-value="addPictures"
        >
          <template v-slot:prepend>
            <q-icon name="attach_file" />
          </template>
        </q-file>
      </ToolPane>
      <PicturesViewer ref="picturesViewer" />
    </div>
  </WidgetPane>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import PicturesViewer from 'src/components/collection/tabs/folder/pictures/PicturesViewer.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const picturesViewer = ref(null)
const files = ref(null)
let folderId = 0

function blobToBase64 (blob) {
  return new Promise((resolve) => {
    const reader = new FileReader()
    reader.onloadend = () => resolve(reader.result)
    reader.readAsDataURL(blob)
  })
}

async function addPicture (file) {
  const dataBase64 = await blobToBase64(file)
  const blob = {
    file_name: file.name,
    file_type: file.type,
    data_base64: dataBase64.substr(dataBase64.indexOf(',') + 1)
  }
  await aminaApi.sendRequest('lappi.collection.pictures.add_blob_to_collection', { blob, folder_id: folderId })
  await picturesViewer.value.update(folderId)
}

async function addPictures (newFiles) {
  for (const file of newFiles) {
    await addPicture(file)
  }
  files.value = null
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
