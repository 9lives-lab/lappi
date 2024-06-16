<template>
  <WidgetPane title="Tags">
    <div class="tag-editor row q-pa-md q-gutter-md">
      <TagField class="col" v-for="item in items" :key="item.key" v-bind:item="item" />
    </div>
  </WidgetPane>
</template>

<script setup>
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import TagField from 'components/collection/common/tags/TagField.vue'
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
