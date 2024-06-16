<template>
  <div class="folder-editor q-gutter-md">
    <TabHeader :itemType="folderType" :title="folderName" />
    <PicturesWidget ref="picturesWidget"/>
  </div>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import TabHeader from 'src/components/collection/tabs/TabHeader.vue'
import PicturesWidget from 'src/components/collection/tabs/folder/pictures/PicturesWidget.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const folderName = ref(null)
const folderType = ref(null)
const picturesWidget = ref(null)

async function updateFolder (newFolderId) {
  const folderDescription = await aminaApi.sendRequest('lappi.collection.folders.get_folder_description', { folder_id: newFolderId })
  folderName.value = folderDescription.name
  folderType.value = folderDescription.folder_type

  await picturesWidget.value.update(newFolderId)
}

defineExpose({
  updateFolder
})
</script>

<style lang="sass" scoped>
.name-field
  text-align: center

</style>
