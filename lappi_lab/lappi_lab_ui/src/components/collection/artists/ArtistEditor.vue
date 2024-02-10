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
    <TitledPane title="Explore">
      <ItemExploringPane ref="itemExploringPane"></ItemExploringPane>
    </TitledPane>
  </div>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import TitledPane from 'src/amina_ui/components/TitledPane.vue'
import ItemExploringPane from 'src/components/collection/ItemExploringPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const itemExploringPane = ref(null)
const artistName = ref(null)

async function updateArtist (newArtistId) {
  const artistDescription = await aminaApi.sendRequest('lappi.collection.artists.get_description', { artist_id: newArtistId })
  artistName.value = artistDescription.name
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
