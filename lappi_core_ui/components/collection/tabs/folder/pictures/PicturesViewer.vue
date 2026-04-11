<template>
  <q-scroll-area
    v-if="picturesUrlList.length > 0"
    class="pictures-scroll-area"
  >
    <div class="pictures-viewer row no-wrap items-center q-pa-md q-gutter-md">
      <div v-for="item in picturesUrlList" :key="item.id">
        <img :src="item.url" class="image-container">
        <q-menu>
          <q-list style="min-width: 100px">
            <q-item>
              <q-item-section>Picture id: {{ item.id }}</q-item-section>
            </q-item>
            <q-separator />
            <q-item clickable v-close-popup @click="setFolderCover(item)">
              <q-item-section>Set folder cover</q-item-section>
            </q-item>
            <q-item clickable v-close-popup @click="deletePicture(item)">
              <q-item-section>Delete</q-item-section>
            </q-item>
          </q-list>
        </q-menu>
      </div>
    </div>
  </q-scroll-area>
</template>

<script setup>
import { getCurrentInstance, ref, onMounted } from 'vue'

const lappiApi = getCurrentInstance().appContext.config.globalProperties.$lappiApi

const picturesUrlList = ref([])
let currentFolderId = -1

async function setFolderCover (item) {
  await lappiApi.sendRequest('lappi.collection.folders.set_folder_cover', { folder_id: currentFolderId, picture_id: item.id })
}

async function deletePicture (item) {
  await lappiApi.sendRequest('lappi.collection.pictures.delete_picture', { picture_id: item.id })
}

async function updateUrls (picturesDescList) {
  const newUrlList = []
  for (const pictureDesc of picturesDescList) {
    const url = await lappiApi.getInternalFileUrl(pictureDesc.internal_file_id)
    newUrlList.push({
      url,
      id: pictureDesc.picture_id
    })
  }
  picturesUrlList.value = newUrlList
}

async function update (newFolderId) {
  currentFolderId = newFolderId
  const picturesDescList = await lappiApi.sendRequest('lappi.collection.pictures.get_pictures_in_folder', { folder_id: newFolderId })
  await updateUrls(picturesDescList)
}

defineExpose({
  update
})

onMounted(async () => {

})
</script>

<style lang="sass" scoped>
.pictures-scroll-area
  height: 225px
  width: 100%

.pictures-viewer
  height: 100%

.image-container
  max-height: 200px
  width: auto
  height: auto
  cursor: pointer
  border-radius: 5px
  border: 1px solid #0c2230
  box-shadow: 0px 0px 8px 0px rgba(0,0,0,0.5)
  transition: 0.3s ease
  &:hover
    filter: brightness(1.1)

</style>
