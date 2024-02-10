<template>
  <TitledPane title="Explore" class="column">
    <div class="column col">
      <ToolPane class="col-auto">
        <q-btn icon="search" label="Find music files" style="color: #60bae0;" @click="findMusicFiles()" />
        <q-btn icon="file_download" label="Import to collection" style="color: #72ba85;" @click="importMusicFIles()" />
      </ToolPane>
      <AbsoluteWrapper class="col">
        <q-table
          style="max-height: 100%;"
          flat dense
          :rows="rows"
          :columns="columns"
          row-key="index"
          virtual-scroll
          v-model:pagination="pagination"
          :rows-per-page-options="[0]"
        />
      </AbsoluteWrapper>
    </div>
  </TitledPane>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import TitledPane from 'src/amina_ui/components/TitledPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import AbsoluteWrapper from 'src/amina_ui/components/AbsoluteWrapper.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const currentPath = ref('')

const tableColumns = ['title', 'album', 'artist']
const columns = [{
  name: 'index',
  label: '#',
  field: 'index',
  align: 'left'
}]
for (const column of tableColumns) {
  columns.push({
    name: column,
    required: true,
    label: column,
    align: 'center',
    field: column
  })
}
columns[1].align = 'left'

const rows = ref([])

const pagination = ref({
  rowsPerPage: 0
})

async function updateCurrentFolder (path) {
  currentPath.value = path
}

async function findMusicFiles () {
  const list = await aminaApi.sendRequest('lappi.file_manager.get_files_in_dir', { path: currentPath.value, recursive: true })

  const newListItems = []
  let id = 0

  for (const file of list) {
    const fileDescription = await aminaApi.sendRequest('lappi.files_explorer.get_file_description', { path: file })
    if (fileDescription.media_type === 'audio') {
      const newItem = {
        index: id,
        file
      }
      let skip = false
      for (const column of tableColumns) {
        if (column in fileDescription.tags) {
          newItem[column] = fileDescription.tags[column].String
        } else {
          skip = true
        }
      }
      if (skip) {
        continue
      }
      newListItems.push(newItem)
      id++
    }
  }

  rows.value = newListItems
}

async function importMusicFIles () {
  for (const row of rows.value) {
    const tags = {}
    for (const column of tableColumns) {
      tags[column] = row[column]
    }
    await aminaApi.sendRequest('lappi.import.import_basic', { tags, file_path: row.file })
  }
  rows.value = []
}

defineExpose({
  updateCurrentFolder
})
</script>

<style lang="sass" scoped>
</style>
