<template>
  <div class="">
    <FolderEditor
      ref="folderEditor"
      v-show="editorType === 'folder'"
    />
    <ItemEditor
      ref="itemEditor"
      v-show="editorType === 'item'"
    />
    <div v-show="editorType === 'none'">None</div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import FolderEditor from 'src/components/collection/folders/FolderEditor.vue'
import ItemEditor from 'src/components/collection/music_items/ItemEditor.vue'

const folderEditor = ref(null)
const itemEditor = ref(null)
const editorType = ref('none')

async function setFolder (folderId) {
  editorType.value = 'folder'
  await folderEditor.value.updateFolder(folderId)
}

async function setItem (itemId) {
  editorType.value = 'item'
  await itemEditor.value.updateItem(itemId)
}

defineExpose({
  setFolder,
  setItem
})
</script>

<style lang="sass" scoped>

</style>
