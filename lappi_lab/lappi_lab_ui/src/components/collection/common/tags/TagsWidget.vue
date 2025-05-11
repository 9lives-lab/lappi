<template>
  <WidgetPane title="Tags">
    <div class="column">
      <ToolPane>
        <div class="q-pa-md">Custom tags</div>
        <AddTagDialog class="q-mr-md" :adapter="adapter" />
      </ToolPane>
      <div class="tag-editor row q-pa-md q-gutter-md">
        <TagField class="tag-fields col-5" v-for="tag in customTags" :key="tag.tagName" :tagAdapter="tag" :readonly="false"/>
        <div v-show="customTags.length === 0">No tags yet</div>
      </div>
      <ToolPane>
        <div class="q-pa-md">Inherited tags</div>
      </ToolPane>
      <div class="tag-editor row q-pa-md q-gutter-md">
        <TagField
          class="tag-fields col-5" v-for="tag in inheritedTags" :key="tag.tagName" :tagAdapter="tag" :readonly="true"/>
        <div v-show="inheritedTags.length === 0">No tags yet</div>
      </div>
    </div>
  </WidgetPane>
</template>

<script setup>
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import TagField from 'components/collection/common/tags/TagField.vue'
import AddTagDialog from 'components/collection/common/tags/AddTagDialog.vue'
import { ref, getCurrentInstance } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const customTags = ref([])
const inheritedTags = ref([])

let adapter = ref({})

function createTagFieldAdapter (adapter, tag) {
  return {
    tagName: tag.key,
    initialValue: tag.value,

    async setValue (newValue) {
      await adapter.setTag(tag.key, newValue)
    },

    async deleteTag () {
      await adapter.deleteTag(tag.key)
    },
  }
}

function createTagFieldAdapters (adapter, tags) {
  return tags.map((tag) => createTagFieldAdapter(adapter, tag))
}

async function update () {
  customTags.value    = createTagFieldAdapters(adapter.value, await adapter.value.getTags())
  inheritedTags.value = createTagFieldAdapters(adapter.value, await adapter.value.getInheritedTags())
}

async function setMusicItem (newMusicItemId) {
  adapter.value = {
    async setTag (tagName, newValue) {
      await aminaApi.sendRequest('lappi.collection.music.set_tag', { item_id: newMusicItemId, tag_name: tagName, tag_value: newValue })
    },
    async deleteTag (tagName) {
      await aminaApi.sendRequest('lappi.collection.music.delete_tag', { item_id: newMusicItemId, tag_name: tagName })
    },
    async getTags () {
      return await aminaApi.sendRequest('lappi.collection.music.get_tags', { item_id: newMusicItemId })
    },
    async getInheritedTags () {
      return await aminaApi.sendRequest('lappi.collection.music.get_inheirted_tags', { item_id: newMusicItemId })
    }
  }
  await update()
}

async function setFolder (newFolderId) {
  adapter.value = {
    async setTag (tagName, newValue) {
      await aminaApi.sendRequest('lappi.collection.folders.set_tag', { folder_id: newFolderId, tag_name: tagName, tag_value: newValue })
    },
    async deleteTag (tagName) {
      await aminaApi.sendRequest('lappi.collection.folders.delete_tag', { folder_id: newFolderId, tag_name: tagName })
    },
    async getTags () {
      return await aminaApi.sendRequest('lappi.collection.folders.get_tags', { folder_id: newFolderId })
    },
    async getInheritedTags () {
      return await aminaApi.sendRequest('lappi.collection.folders.get_inheirted_tags', { folder_id: newFolderId })
    }
  }
  await update()
}

defineExpose({
  setMusicItem,
  setFolder
})
</script>

<style lang="sass" scoped>
.tag-field
  width: 45%

</style>
