<template>
  <WidgetPane title="Summary">
    <div class="row">
      <div class="column col q-pa-md q-gutter-md">
        <div class="row items-center">
          <div class="col-auto q-pr-md">Folder name:</div>
          <q-input
            dense square filled
            class="col"
            v-model="folderName"
            @update:model-value="setName"
          />
        </div>
        <div class="row items-center">
          <div class="col-auto q-pr-md">Folder type:</div>
          <q-select
            dense square standout
            class="col"
            v-model="folderType"
            :options="folderTypeOptions"
            @update:model-value="setFolderType"
          />
        </div>
      </div>
      <div class="column col q-pa-md q-gutter-md">
        <div class="row items-center">
          <div class="col-auto q-pr-md">Folder ID:</div>
          <q-input
            dense square filled readonly
            class="col"
            v-model="folderId"
          />
        </div>
      </div>
    </div>
    <div class="row items-center q-pl-md q-pr-md q-pb-md">
      <div class="col-auto q-pr-md">Internal path:</div>
      <q-input
        dense square filled readonly
        class="col"
        v-model="internalPath"
      />
    </div>
  </WidgetPane>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const folderId = ref(0)
const folderName = ref('')
const folderType = ref(null)
const folderTypeOptions = ['Folder', 'Artist', 'Album']
const internalPath = ref('')

async function update (newFolderId) {
  folderId.value = newFolderId

  const folderDescription = await aminaApi.sendRequest('lappi.collection.folders.get_folder_description', { folder_id: newFolderId })
  folderName.value = folderDescription.name
  folderType.value = folderDescription.folder_type

  const newInternalPath = await aminaApi.sendRequest('lappi.collection.folders.get_internal_path', { folder_id: newFolderId })
  internalPath.value = newInternalPath.path
}

async function setName (newName) {
  await aminaApi.sendRequest('lappi.collection.folders.set_folder_name', { folder_id: folderId.value, name: newName })
}

async function setFolderType (newFolderType) {
  await aminaApi.sendRequest('lappi.collection.folders.set_folder_type', { folder_id: folderId.value, folder_type: newFolderType })
}

defineExpose({
  update
})
</script>

<style lang="sass" scoped>

</style>
