<template>
  <WidgetPane title="External sources">
    <div class="column playback-sources">
      <ToolPane class="col-auto">
        <q-btn icon="add" label="Add External file" @click="addMusicLink('ExternalFile')" />
        <q-btn icon="add" label="Add URL" @click="addMusicLink('Url')" />
      </ToolPane>
      <q-list separator v-if="links.length > 0">
        <q-item clickable v-ripple v-for="linkDesc in links" :key="linkDesc.id">
          <q-item-section side>
            <q-item-label>{{ linkDesc.link_type }}</q-item-label>
          </q-item-section>
          <q-item-section>
            <q-input
              dense square filled
              v-model="linkDesc.link"
              @update:model-value="newValue => setMusicLink(newValue, linkDesc)"
            />
          </q-item-section>
          <q-item-section side>
            <q-btn class="delete-button" icon="delete" @click="deleteMusicLink(linkDesc)" />
          </q-item-section>
        </q-item>
      </q-list>
      <div v-else class="col text-center q-pa-md">
        No external sources
      </div>
    </div>
  </WidgetPane>
</template>

<script setup>
import { getCurrentInstance, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

let currentItemId = -1
const links = ref([])

async function updateItem (itemId) {
  currentItemId = itemId
  const newLinks = await aminaApi.sendRequest('lappi.collection.music_sources.get_music_links', { item_id: currentItemId })
  links.value = newLinks.map(linkDesc => ({
    id: linkDesc.id,
    link: linkDesc.link,
    link_type: linkDesc.link_type
  }))
}

async function addMusicLink (link_type) {
  await aminaApi.sendRequest('lappi.collection.music_sources.add_music_link', { item_id: currentItemId, link_type: link_type, link: '' })
}

async function setMusicLink (newValue, linkDesc) {
  await aminaApi.sendRequest('lappi.collection.music_sources.set_music_link', { link_id: linkDesc.id, link: newValue })
}

async function deleteMusicLink (linkDesc) {
  await aminaApi.sendRequest('lappi.collection.music_sources.delete_music_link', { link_id: linkDesc.id })
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>
.delete-button
  color: $amina-negative

</style>
