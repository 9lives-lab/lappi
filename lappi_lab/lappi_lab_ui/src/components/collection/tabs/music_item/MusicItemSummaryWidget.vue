<template>
  <WidgetPane title="Summary">
    <div class="row">
      <div class="column col q-pa-md q-gutter-md">
        <div class="row items-center">
          <div class="col-auto q-pr-md">Item name:</div>
          <q-input
            dense square filled
            class="col"
            v-model="itemName"
            @update:model-value="setName"
          />
        </div>
      </div>
      <div class="column col q-pa-md q-gutter-md">
        <div class="row items-center">
          <div class="col-auto q-pr-md">Music item ID: {{ itemId }}</div>
        </div>
      </div>
    </div>
  </WidgetPane>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const itemId = ref(0)
const itemName = ref('')

async function update (newItemId) {
  itemId.value = newItemId
  const folderDescription = await aminaApi.sendRequest('lappi.collection.music.get_item_description', { item_id: newItemId })
  itemName.value = folderDescription.name
}

async function setName (newName) {
  await aminaApi.sendRequest('lappi.collection.music.set_item_name', { item_id: itemId.value, name: newName })
}

defineExpose({
  update
})
</script>

<style lang="sass" scoped>

</style>
