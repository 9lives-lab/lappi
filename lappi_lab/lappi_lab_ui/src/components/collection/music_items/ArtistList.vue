<template>
  <div class="row q-gutter-sm">
    <q-item
      v-for="item in items"
      :key="item.name"
      clickable
      class="artist-item"
    >
      <q-item-section avatar>
        <q-icon name="account_circle" />
      </q-item-section>
      <q-item-section>
        <q-item-label>{{ item.name }}</q-item-label>
        <q-item-label caption>Band</q-item-label>
      </q-item-section>
    </q-item>
  </div>
</template>

<script setup>
import { ref, getCurrentInstance } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const items = ref([])

async function updateItem (itemId) {
  const idList = await aminaApi.sendRequest('lappi.collection.music.get_artists', { item_id: itemId })
  const newItems = []
  for (const i in idList) {
    const artistDescription = await aminaApi.sendRequest('lappi.collection.artists.get_description', { artist_id: idList[i] })
    newItems.push(artistDescription)
  }
  items.value = newItems
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>
.artist-item
  margin-top: 0px
</style>
