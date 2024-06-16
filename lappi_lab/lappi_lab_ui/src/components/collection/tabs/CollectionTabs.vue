<template>
  <div class="column">
    <div v-show="tabType === 'root'" class="root-tab col"></div>
    <AbsoluteWrapper v-show="tabType !== 'root'" class="col ">
      <q-scroll-area style="height: 100%; max-width: 100%;">
        <FolderTab v-show="tabType === 'folder'" ref="folderTab" />
        <MusicItemTab v-show="tabType === 'music_item'" ref="musicItemTab" />
      </q-scroll-area>
    </AbsoluteWrapper>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import AbsoluteWrapper from 'src/amina_ui/components/AbsoluteWrapper.vue'
import FolderTab from 'src/components/collection/tabs/folder/FolderTab.vue'
import MusicItemTab from 'src/components/collection/tabs/music_item/MusicItemTab.vue'

const folderTab = ref(null)
const musicItemTab = ref(null)
const tabType = ref('root')

async function setFolder (folderId) {
  if (folderId === 0) {
    tabType.value = 'root'
  } else {
    tabType.value = 'folder'
    await folderTab.value.updateFolder(folderId)
  }
}

async function setItem (itemId) {
  tabType.value = 'music_item'
  await musicItemTab.value.updateItem(itemId)
}

defineExpose({
  setFolder,
  setItem
})
</script>

<style lang="sass" scoped>
.root-tab
  height: 100%
  width: 100%
  background-image: url( '~assets/lappi_pattern.svg' )
  background-position: center
  background-size: 350px
  background-repeat: no-repeat

</style>
