<template>
  <q-breadcrumbs active-color="white" style="font-size: 16px">
    <template v-slot:separator>
      <q-icon
        size="1.5em"
        name="chevron_right"
        color="secondary"
      />
    </template>

    <q-breadcrumbs-el
      label="Root"
      v-on:click="onFolderClicked(0)"
      class="folder-button"
    />
    <q-breadcrumbs-el
      v-for="item in items"
      :key="item.folder_id"
      :label="item.title"
      class="folder-button"
      v-on:click="onFolderClicked(item.folder_id)"
    />
  </q-breadcrumbs>
</template>

<script setup>
import { ref, getCurrentInstance } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const emit = defineEmits(['folder-selected'])
const items = ref([])

async function update (folderId) {
  const parentFolders = await aminaApi.sendRequest('lappi.collection.view.get_parent_folders', { folder_id: folderId })
  items.value = parentFolders
}

function onFolderClicked (folderId) {
  emit('folder-selected', folderId)
}

defineExpose({
  update
})
</script>

<style lang="sass" scoped>

.folder-button
  cursor: pointer
  &:hover
    text-shadow: #5e929c50 0px 0 10px

</style>
