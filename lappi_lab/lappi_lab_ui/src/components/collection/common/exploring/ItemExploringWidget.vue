<template>
  <div class="column no-wrap item-exploring-pane">
    <ToolPane class="controls-pane">
      <q-select dense filled v-model="sourceListModel" :options="sourceListOptions" label="Source" />
      <q-btn label="Explore" @click="explore()"></q-btn>
    </ToolPane>
    <ItemDescriptionContent class="col" ref="itemDescriptionContent"/>
  </div>
</template>

<script setup>
import { getCurrentInstance, onMounted, ref } from 'vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import ItemDescriptionContent from './ItemDescriptionContent.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const itemDescriptionContent = ref(null)
const sourceListModel = ref(null)
const sourceListOptions = ref([])
const artistName = ref(null)

async function updateSourceList () {
  const sourceList = await aminaApi.sendRequest('lappi.exploring.get_source_list')
  sourceListOptions.value = sourceList
  sourceListModel.value = sourceList[0]
}

async function updateArtist (newArtistId) {
  const artistDescription = await aminaApi.sendRequest('lappi.collection.artists.get_description', { artist_id: newArtistId })
  artistName.value = artistDescription.name
  await updateSourceList()
  itemDescriptionContent.value.clear()
}

async function explore () {
  const artistDescription = await aminaApi.sendRequest('lappi.exploring.get_artist_description', {
    source_name: sourceListModel.value,
    artist_name: artistName.value
  })
  itemDescriptionContent.value.setText(artistDescription)
}

onMounted(async () => {
  await updateSourceList()
})

defineExpose({
  updateArtist
})
</script>

<style lang="sass" scoped>
/* Your styles here */
</style>
