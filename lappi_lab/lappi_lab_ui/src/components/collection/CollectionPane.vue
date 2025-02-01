<template>
  <div class="flex column q-pa-sm">
    <div class="row col-auto q-pb-sm q-pl-sm">
      <NavigationBar ref="navigationBar" v-on:folder-selected="openFolder($event)" />
    </div>
    <div class="row col q-gutter-md">
      <WidgetPane class="col-3">
        <AbsoluteWrapper class="list-wrapper col">
          <q-virtual-scroll class="list-scroll" :items="listItems" v-slot="{ item, index }" ref="listScroll">
            <q-item class="list-item" :key="index" clickable @click="onItemClicked(item)">
              <q-item-section avatar>
                <q-avatar v-show="item.hasAvatar === true" rounded>
                  <img :src="item.pictureUrl">
                </q-avatar>
                <q-icon v-show="item.hasAvatar === false" :name="item.icon" />
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ item.title }}</q-item-label>
                <q-item-label caption>{{ item.caption }}</q-item-label>
              </q-item-section>
            </q-item>
          </q-virtual-scroll>
        </AbsoluteWrapper>
        <ToolPane class="col-auto">
          <q-btn label="Add Item" @click="isNewItemDialogOpened = true" />
          <q-dialog v-model="isNewItemDialogOpened" persistent>
            <q-card style="min-width: 350px">
              <q-card-section>
                <div class="text-h7">Add item</div>
              </q-card-section>
              <q-card-section class="q-pt-none">
                <q-input dense v-model="newItemName" autofocus @keyup.enter="isNewItemDialogOpened = false" />
              </q-card-section>
              <q-card-actions align="right" class="text-primary">
                <q-btn flat label="Cancel" v-close-popup />
                <q-btn flat label="Add item" v-close-popup @click="addItem()" />
              </q-card-actions>
            </q-card>
          </q-dialog>
          <q-btn label="Add Folder" @click="isNewFolderDialogOpened = true" ></q-btn>
          <q-dialog v-model="isNewFolderDialogOpened" persistent>
            <q-card style="min-width: 350px">
              <q-card-section>
                <div class="text-h7">Add folder</div>
              </q-card-section>
              <q-card-section class="q-pt-none">
                <q-input dense v-model="newFolderName" autofocus @keyup.enter="isNewFolderDialogOpened = false" />
              </q-card-section>
              <q-card-actions align="right" class="text-primary">
                <q-btn flat label="Cancel" v-close-popup />
                <q-btn flat label="Add folder" v-close-popup @click="addFolder()" />
              </q-card-actions>
            </q-card>
          </q-dialog>
        </ToolPane>
      </WidgetPane>
      <CollectionTabs class="col" ref="collectionTabs" />
    </div>
  </div>
</template>

<script setup>
import { getCurrentInstance, onMounted, onUnmounted, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import AbsoluteWrapper from 'src/amina_ui/components/AbsoluteWrapper.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import NavigationBar from 'src/components/collection/NavigationBar.vue'
import CollectionTabs from 'src/components/collection/tabs/CollectionTabs.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const isNewFolderDialogOpened = ref(false)
const isNewItemDialogOpened = ref(false)

const listScroll = ref(null)
const navigationBar = ref(null)
const collectionTabs = ref(null)
const listItems = ref([])
const newItemName = ref('')
const newFolderName = ref('')
let currentFolderId = 0
let currentItemId = -1

async function getFolderItem (id, folderDescription) {
  const item = {
    id,
    folder_id: folderDescription.folder_id,
    title: folderDescription.name,
    caption: folderDescription.folder_type
  }

  if ('avatar_picture_id' in folderDescription) {
    const path = await aminaApi.sendRequest('lappi.collection.pictures.get_picture_path', { picture_id: folderDescription.avatar_picture_id })
    const pictureUrl = await aminaApi.getFileUrl(path)

    item.hasAvatar = true
    item.pictureUrl = pictureUrl
  } else {
    let icon = 'folder_open'

    switch (folderDescription.folder_type) {
      case 'Artist':
        icon = 'account_circle'
        break
      case 'Album':
        icon = 'album'
        break
    }

    item.hasAvatar = false
    item.icon = icon
  }

  const caption = await aminaApi.sendRequest('lappi.collection.folders.get_folder_caption', { folder_id: folderDescription.folder_id })
  if (caption !== '') {
    item.caption = item.caption + ', ' + caption
  }

  return item
}

async function getMusicItem (id, itemDescription) {
  const item = {
    id,
    item_id: itemDescription.item_id,
    title: itemDescription.name,
    hasAvatar: false,
    icon: 'library_music',
    caption: 'Song'
  }
  const caption = await aminaApi.sendRequest('lappi.collection.music.get_item_caption', { item_id: itemDescription.item_id })
  if (caption !== '') {
    item.caption = item.caption + ', ' + caption
  }

  return item
}

async function updateFolders (folderId) {
  const { content } = await aminaApi.sendRequest('lappi.collection.folders.get_folder_content', { folder_id: folderId })

  const folders = await Promise.all(content.folders.map(async (folder, id) => (await getFolderItem(id, folder))))
  const items = await Promise.all(content.items.map(async (item, id) => (await getMusicItem(id + content.folders.length, item))))

  listItems.value = [...folders, ...items]
  navigationBar.value.update(folderId)
}

async function openFolder (folderId) {
  await updateFolders(folderId)
  collectionTabs.value.setFolder(folderId)
  if (currentFolderId !== folderId) {
    listScroll.value.scrollTo({ top: 0 })
  }
  currentFolderId = folderId
  currentItemId = -1
}

async function updateItem (itemId) {
  collectionTabs.value.setItem(itemId)
}

async function onItemClicked (item) {
  if ('folder_id' in item) {
    await openFolder(item.folder_id)
  } else {
    await updateItem(item.item_id)
    currentItemId = item.item_id
  }
}

async function update () {
  await updateFolders(currentFolderId)
  if (currentItemId >= 0) {
    await updateItem(currentItemId)
  } else {
    await openFolder(currentFolderId)
  }
}

async function addItem () {
  await aminaApi.sendRequest('lappi.collection.music.create_item', { name: newItemName.value, folder_id: currentFolderId })
  newItemName.value = ''
}

async function addFolder () {
  await aminaApi.sendRequest('lappi.collection.folders.find_or_add_folder', { parent_id: currentFolderId, folder_name: newFolderName.value, folder_type: 'Folder' })
  newFolderName.value = ''
}

onMounted(() => {
  aminaApi.setEventHandler('lappi.collection.OnCollectionUpdated', 'CollectionPane', () => {
    update()
  })

  openFolder(0)
})

onUnmounted(() => {
  aminaApi.removeEventHandler('lappi.collection.OnCollectionUpdated', 'CollectionPane')
})
</script>

<style lang="sass" scoped>

.q-item__section--avatar
  min-width: 0px

.list-scroll
  max-height: 100%

.list-item
  border-style: solid
  border-width: 0px 0px 1px 0px
  border-color: $amina-separator-color

</style>
