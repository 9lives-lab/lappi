<template>
  <WidgetPane title="Tags">
    <div class="column">
      <ToolPane>
        <div class="q-pa-md">Custom tags</div>
        <q-btn icon="add" class="q-mr-md" @click="addTag" />
        <q-input borderless dense placeholder="Add tag" v-model="newTagName" />
      </ToolPane>
      <div class="tag-editor row q-pa-md q-gutter-md">
        <TagField
          class="tag-fields col-5"
          v-for="tag in customTags"
          :key="tag.key"
          :name="tag.key"
          v-model="tag.value.String"
          @input="setTag(tag.key, tag.value.String)"
        />
      </div>
      <ToolPane>
        <div class="q-pa-md">Inherited tags</div>
      </ToolPane>
      <div class="tag-editor row q-pa-md q-gutter-md">
        <TagField
          class="tag-fields col-5"
          v-for="tag in inheritedTags"
          :key="tag.key"
          :name="tag.key"
          v-model="tag.value.String"
        />
      </div>
    </div>
  </WidgetPane>
</template>

<script setup>
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import TagField from 'components/collection/common/tags/TagField.vue'
import { ref, getCurrentInstance } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const customTags = ref([])
const inheritedTags = ref([])

const newTagName = ref('')

let adapter = null

async function setTag (tagName, newValue) {
  await adapter.setTag(tagName, newValue)
}

async function addTag () {
  await setTag(newTagName.value, '')
  newTagName.value = ''
}

async function update () {
  customTags.value = await adapter.getTags()
  inheritedTags.value = await adapter.getInheritedTags()
}

async function setMusicItem (newMusicItemId) {
  adapter = {
    async setTag (tagName, newValue) {
      await aminaApi.sendRequest('lappi.collection.music.set_tag', { item_id: newMusicItemId, tag_name: tagName, tag_value: newValue })
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
  adapter = {
    async setTag (tagName, newValue) {
      await aminaApi.sendRequest('lappi.collection.folders.set_tag', { folder_id: newFolderId, tag_name: tagName, tag_value: newValue })
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
