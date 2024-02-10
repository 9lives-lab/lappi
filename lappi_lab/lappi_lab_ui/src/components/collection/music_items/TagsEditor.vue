<template>
  <div class="tag-editor row q-gutter-sm">
    <TagField class="col" v-for="item in items" :key="item.key" v-bind:item="item" />
  </div>
</template>

<script setup>
import TagField from 'components/collection/music_items/TagField.vue'
import { ref, getCurrentInstance } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const items = ref([])

async function updateItem (itemId) {
  const tags = await aminaApi.sendRequest('lappi.collection.get_tags', { item_id: itemId })
  items.value = tags
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>

</style>
