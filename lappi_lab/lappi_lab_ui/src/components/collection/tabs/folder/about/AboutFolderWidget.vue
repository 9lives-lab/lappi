<template>
  <WidgetPane title="About">
    <div class="column about-folder-widget">
      <q-editor
        v-model="text"
        min-height="5rem"
        flat
        square
        :definitions="{
          save: {
            tip: 'Save',
            icon: 'save',
            label: 'Save',
            handler: saveText
          }
        }"
        :toolbar="[
          ['bold', 'italic', 'strike', 'underline'],
          ['quote', 'unordered', 'ordered', 'outdent', 'indent'],
          ['undo', 'redo'],
          ['save']
        ]"
      />
    </div>
  </WidgetPane>
</template>

<script setup>
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import { ref, getCurrentInstance } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const text = ref('')
let currentFolderId = -1

async function update (folderId) {
  currentFolderId = folderId
  const newText = await aminaApi.sendRequest('lappi.collection.folders.get_description', { folder_id: currentFolderId })
  text.value = newText
}

async function saveText () {
  if (currentFolderId >= 0) {
    await aminaApi.sendRequest('lappi.collection.folders.save_description', { folder_id: currentFolderId, text: text.value })
  }
}

defineExpose({
  update
})
</script>
