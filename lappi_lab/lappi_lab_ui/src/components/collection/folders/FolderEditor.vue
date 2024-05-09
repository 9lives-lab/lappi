<template>
  <div class="folder-editor">
    <EditorHeader :itemType="folderType" :title="folderName" />
    <TitledPane title="Pictures">
      <PicturesEditor ref="picturesEditor"/>
    </TitledPane>
    <TitledPane title="Explore">
      <ItemExploringPane ref="itemExploringPane" />
    </TitledPane>
  </div>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import TitledPane from 'src/amina_ui/components/TitledPane.vue'
import EditorHeader from 'src/components/collection/EditorHeader.vue'
import PicturesEditor from 'src/components/collection/pictures/PicturesEditor.vue'
import ItemExploringPane from 'src/components/collection/ItemExploringPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const folderName = ref(null)
const folderType = ref(null)
const picturesEditor = ref(null)
const itemExploringPane = ref(null)

async function updateFolder (newFolderId) {
  const folderDescription = await aminaApi.sendRequest('lappi.collection.folders.get_folder_description', { folder_id: newFolderId })
  folderName.value = folderDescription.name
  folderType.value = folderDescription.folder_type

  await picturesEditor.value.update(newFolderId)

  await itemExploringPane.value.updateArtist(newFolderId)
}

defineExpose({
  updateFolder
})
</script>

<style lang="sass" scoped>
.name-field
  text-align: center

</style>
