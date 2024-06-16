<template>
  <q-scroll-area
    v-if="picturesUrlList.length > 0"
    class="pictures-scroll-area"
  >
    <div class="pictures-viewer row no-wrap items-center q-pa-md q-gutter-md">
      <img
        v-for="item in picturesUrlList"
        :key="item.id"
        :src="item.url"
        class="image-container"
      />
    </div>
  </q-scroll-area>
</template>

<script setup>
import { getCurrentInstance, ref, onMounted } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const picturesUrlList = ref([])

async function updateUrls (picturesIdList) {
  const newUrlList = []
  for (const pictureId of picturesIdList) {
    const path = await aminaApi.sendRequest('lappi.collection.pictures.get_picture_path', { picture_id: pictureId })
    const url = await aminaApi.getFileUrl(path)
    newUrlList.push({
      url,
      id: pictureId
    })
  }
  picturesUrlList.value = newUrlList
}

async function update (newFolderId) {
  const picturesIdList = await aminaApi.sendRequest('lappi.collection.pictures.get_pictures_in_folder', { folder_id: newFolderId })
  await updateUrls(picturesIdList)
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
  box-shadow: 0px 0px 15px 0px rgba(0,0,0,0.8)
  transition: 0.3s ease
  &:hover
    filter: brightness(1.1)

</style>
