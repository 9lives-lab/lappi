<template>
  <div class="artist-editor">
    <TitledPane title="Artist">
      <div class="row">
        <q-input
          dense
          borderless
          class="name-field col"
          input-style="text-align: center"
          v-model="artistName"
        />
      </div>
    </TitledPane>
    <TitledPane title="Pictures">
      <PicturesEditor ref="picturesEditor"/>
    </TitledPane>
    <TitledPane title="Explore">
      <ItemExploringPane ref="itemExploringPane" />
    </TitledPane>
  </div>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import TitledPane from 'src/amina_ui/components/TitledPane.vue'
import PicturesEditor from 'src/components/collection/pictures/PicturesEditor.vue'
import ItemExploringPane from 'src/components/collection/ItemExploringPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const artistName = ref(null)
const picturesEditor = ref(null)
const itemExploringPane = ref(null)

async function updateArtist (newArtistId) {
  const artistDescription = await aminaApi.sendRequest('lappi.collection.artists.get_description', { artist_id: newArtistId })
  artistName.value = artistDescription.name

  await picturesEditor.value.update({
    id: newArtistId,
    getPicturesRpcKey: 'lappi.collection.pictures.get_pictures_by_artist',
    addPictureRpcKey: 'lappi.collection.pictures.add_picture_to_artist'
  })

  await itemExploringPane.value.updateArtist(newArtistId)
}

defineExpose({
  updateArtist
})
</script>

<style lang="sass" scoped>
.name-field
  text-align: center

</style>
