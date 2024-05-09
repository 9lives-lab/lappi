<template>
  <q-page class="flex column q-pa-sm">
    <div class="row col-auto q-pb-sm q-pl-sm">
      <NavigationBar ref="navigationBar" v-on:folder-selected="openFolder($event)" />
    </div>
    <div class="row col q-gutter-sm">
      <UiPlate class="column col-3">
        <div class="list-wrapper col">
          <div class="list-absolute-wrapper">
            <q-virtual-scroll
              style="max-height: 100%;"
              :items="listItems"
              v-slot="{ item, index }"
            >
              <q-item
                :key="index"
                clickable
                @click="onItemClicked(item)"
              >
                <q-item-section avatar>
                  <q-avatar v-show="item.hasAvatar === true"  rounded>
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
          </div>
        </div>
        <ToolPane class="col-auto">
          <q-btn label="Add Artist" @click="prompt = true" ></q-btn>
          <q-btn label="Add Item" />

          <q-dialog v-model="prompt" persistent>
            <q-card style="min-width: 350px">
              <q-card-section>
                <div class="text-h7">Add artist</div>
              </q-card-section>

              <q-card-section class="q-pt-none">
                <q-input dense v-model="newArtistName" autofocus @keyup.enter="prompt = false" />
              </q-card-section>

              <q-card-actions align="right" class="text-primary">
                <q-btn flat label="Cancel" v-close-popup />
                <q-btn flat label="Add artist" v-close-popup @click="addArtist()" />
              </q-card-actions>
            </q-card>
          </q-dialog>
        </ToolPane>
      </UiPlate>
      <UiPlate class="col">
        <CollectionEditor ref="collectionEditor" />
      </UiPlate>
    </div>
  </q-page>
</template>

<script setup>
import { getCurrentInstance, onMounted, ref } from 'vue'
import UiPlate from 'src/amina_ui/components/UiPlate.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import NavigationBar from 'components/collection/NavigationBar.vue'
import CollectionEditor from 'pages/collection_tree/CollectionEditor.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const prompt = ref(false)

const navigationBar = ref(null)
const collectionEditor = ref(null)
const listItems = ref([])
const newArtistName = ref('')
let currentFolderId = 0

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

  return item
}

async function openFolder (folderId) {
  const { content } = await aminaApi.sendRequest('lappi.collection.folders.get_folder_content', { folder_id: folderId })

  const folders = await Promise.all(content.folders.map(async (folder, id) => (await getFolderItem(id, folder))))

  const items = content.items.map((item, id) => ({
    id: id + content.folders.length,
    item_id: item.item_id,
    title: item.name,
    hasAvatar: false,
    icon: 'library_music',
    caption: 'Song'
  }))

  listItems.value = [...folders, ...items]
  navigationBar.value.update(folderId)
  collectionEditor.value.setFolder(folderId)
  currentFolderId = folderId
}

async function onItemClicked (item) {
  if ('folder_id' in item) {
    await openFolder(item.folder_id)
  } else {
    collectionEditor.value.setItem(item.item_id)
  }
}

async function update () {
  await openFolder(currentFolderId)
}

aminaApi.setEventHandler('lappi.collection.folders.OnFoldersUpdated', (event) => {
  update()
})

async function addArtist () {
  const newFolderId = await aminaApi.sendRequest('lappi.collection.folders.find_or_add_folder', { parent_id: currentFolderId, folder_name: newArtistName.value, folder_type: 'Artist' })
  newArtistName.value = ''
  console.log(newFolderId)
}

onMounted(() => {
  openFolder(0)
})
</script>

<style lang="sass" scoped>

.q-item__section--avatar
  min-width: 0px

.plate
  background-color: $plate-dark-background
  border-radius: 6px
  border-color: $separator-dark-color
  border-width: 1px
  border-style: solid
  box-shadow: rgba(0, 0, 0, 0.14) 0px 0px 12px

.list-wrapper
  overflow: hidden
  position: relative

.list-absolute-wrapper
  overflow: hidden
  position: absolute
  top: 0px
  bottom: 0px
  left: 0px
  right: 0px

</style>
