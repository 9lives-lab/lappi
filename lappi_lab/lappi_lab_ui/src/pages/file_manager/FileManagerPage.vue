<template>
  <q-page class="flex column q-pa-sm">
    <div class="row col-auto q-pb-sm q-pl-sm">
      {{ currentPath }}
    </div>
    <div class="row col q-gutter-md">
      <WidgetPane class="column col-3">
        <ToolPane class="col-auto">
          <q-btn icon="arrow_upward" @click="openParentFolder()"/>
          <q-btn icon="home" @click="openRootFolder()" />
        </ToolPane>
        <AbsoluteWrapper class="col">
          <q-virtual-scroll style="max-height: 100%;" :items="filesList" v-slot="{ item }">
            <q-item :key="item.id" clickable @click="onItemClicked(item)">
              <q-item-section avatar>
                <q-icon :name="item.icon" />
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ item.name }}</q-item-label>
              </q-item-section>
            </q-item>
          </q-virtual-scroll>
        </AbsoluteWrapper>
      </WidgetPane>
      <WidgetPane class="column col" title="Explore">
        <FolderExploringPane class="col" ref="folderExploringPane" />
      </WidgetPane>
    </div>
  </q-page>
</template>

<script setup>
import { getCurrentInstance, onMounted, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import AbsoluteWrapper from 'src/amina_ui/components/AbsoluteWrapper.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import FolderExploringPane from 'src/pages/file_manager/FolderExploringPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const folderExploringPane = ref(null)
const filesList = ref([])
const currentPath = ref('/')

async function openFolder (path) {
  const list = await aminaApi.sendRequest('lappi.file_manager.get_files_list', { path })

  const folders = list.folders.map((folder, id) => ({
    id,
    name: folder.split('/').pop(),
    path: folder,
    icon: 'folder_open'
  }))

  const files = list.files.map((file, id) => ({
    id: id + list.folders.length,
    name: file.split('/').pop(),
    path: file,
    icon: 'description'
  }))

  filesList.value = [...folders, ...files]
  currentPath.value = path

  folderExploringPane.value.updateCurrentFolder(path)
}

async function openParentFolder () {
  const path = currentPath.value
  const parentPath = await aminaApi.sendRequest('lappi.file_manager.get_parent_folder', { path })
  await openFolder(parentPath)
}

async function openRootFolder () {
  await openFolder('/')
}

async function onItemClicked (item) {
  if (item.icon === 'folder_open') {
    await openFolder(item.path)
  }
}

onMounted(() => {
  openRootFolder()
})
</script>

<style lang="sass" scoped>

.q-item__section--avatar
  min-width: 0px

</style>
