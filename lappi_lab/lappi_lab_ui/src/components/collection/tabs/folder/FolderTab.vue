<template>
  <div class="q-gutter-md">
    <TabHeader :itemType="folderType" :title="folderName" />
    <PicturesWidget ref="picturesWidget" />
    <TagsWidget ref="tagsWidget"/>
    <ChatExploringWidget ref="chatExploringWidget" />
    <AboutFolderWidget ref="aboutFolderWidget" />
    <FolderSummaryWidget ref="folderSummaryWidget" />
  </div>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import TabHeader from 'src/components/collection/tabs/TabHeader.vue'
import TagsWidget from 'src/components/collection/common/tags/TagsWidget.vue'
import FolderSummaryWidget from 'src/components/collection/tabs/folder/FolderSummaryWidget.vue'
import PicturesWidget from 'src/components/collection/tabs/folder/pictures/PicturesWidget.vue'
import AboutFolderWidget from 'src/components/collection/tabs/folder/about/AboutFolderWidget.vue'
import ChatExploringWidget from 'src/components/collection/common/exploring/ChatExploringWidget.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const folderName = ref(null)
const folderType = ref(null)

const picturesWidget = ref(null)
const tagsWidget = ref(null)
const folderSummaryWidget = ref(null)
const aboutFolderWidget = ref(null)
const chatExploringWidget = ref(null)

async function updateFolder (newFolderId) {
  const folderDescription = await aminaApi.sendRequest('lappi.collection.folders.get_folder_description', { folder_id: newFolderId })
  folderName.value = folderDescription.name
  folderType.value = folderDescription.folder_type

  await picturesWidget.value.update(newFolderId)
  await tagsWidget.value.setFolder(newFolderId)
  await folderSummaryWidget.value.update(newFolderId)
  await aboutFolderWidget.value.update(newFolderId)
  await chatExploringWidget.value.update({ Folder: newFolderId })
}

defineExpose({
  updateFolder
})
</script>

<style lang="sass" scoped>

</style>
