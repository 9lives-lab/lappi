<template>
  <div class="item-editor col q-gutter-md">
    <TabHeader itemType="" :title="itemName" />
    <TagsWidget ref="tagsWidget"/>
    <LyricsWidget ref="lyricsWidget" />
    <ChatExploringWidget ref="chatExploringWidget" />
    <PlaybackSourcesWidget ref="playbackSourcesWidget" />
  </div>
</template>

<script setup>
import { ref, getCurrentInstance } from 'vue'
import TabHeader from 'src/components/collection/tabs/TabHeader.vue'
import TagsWidget from 'src/components/collection/common/tags/TagsWidget.vue'
import PlaybackSourcesWidget from 'src/components/collection/tabs/music_item/PlaybackSourcesWidget.vue'
import LyricsWidget from 'src/components/collection/tabs/music_item/lyrics/LyricsWidget.vue'
import ChatExploringWidget from 'src/components/collection/common/exploring/ChatExploringWidget.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const tagsWidget = ref(null)
const lyricsWidget = ref(null)
const playbackSourcesWidget = ref(null)
const chatExploringWidget = ref(null)

const itemName = ref('null')

async function updateItem (itemId) {
  const itemDescription = await aminaApi.sendRequest('lappi.collection.music.get_item_description', { item_id: itemId })
  itemName.value = itemDescription.name

  await tagsWidget.value.updateItem(itemId)
  await lyricsWidget.value.updateItem(itemId)
  await chatExploringWidget.value.update({ MusicItem: itemId })
  await playbackSourcesWidget.value.updateItem(itemId)
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>

</style>
