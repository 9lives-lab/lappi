<template>
  <div class="item-editor col q-gutter-md">
    <TabHeader itemType="" :title="itemName" />
    <TagsWidget ref="tagsWidget"/>
    <ChatExploringWidget ref="chatExploringWidget" />
    <LyricsWidget ref="lyricsWidget" />
    <PlaybackSourcesWidget ref="playbackSourcesWidget" />
    <MusicItemSummaryWidget ref="musicItemSummaryWidget" />
    <ScriptsWidget ref="scriptsWidget" />
  </div>
</template>

<script setup>
import { ref, getCurrentInstance } from 'vue'
import TabHeader from 'src/components/collection/tabs/TabHeader.vue'
import TagsWidget from 'src/components/collection/common/tags/TagsWidget.vue'
import MusicItemSummaryWidget from 'src/components/collection/tabs/music_item/MusicItemSummaryWidget.vue'
import PlaybackSourcesWidget from 'src/components/collection/tabs/music_item/PlaybackSourcesWidget.vue'
import LyricsWidget from 'src/components/collection/tabs/music_item/lyrics/LyricsWidget.vue'
import ChatExploringWidget from 'src/components/collection/common/exploring/ChatExploringWidget.vue'
import ScriptsWidget from 'src/components/collection/common/ScriptsWidget.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const tagsWidget = ref(null)
const musicItemSummaryWidget = ref(null)
const lyricsWidget = ref(null)
const playbackSourcesWidget = ref(null)
const chatExploringWidget = ref(null)
const scriptsWidget = ref(null)

const itemName = ref('null')

async function updateItem (itemId) {
  const itemDescription = await aminaApi.sendRequest('lappi.collection.music.get_item_description', { item_id: itemId })
  itemName.value = itemDescription.name

  await tagsWidget.value.updateItem(itemId)
  await musicItemSummaryWidget.value.update(itemId)
  await lyricsWidget.value.updateItem(itemId)
  await chatExploringWidget.value.update({ MusicItem: itemId })
  await playbackSourcesWidget.value.updateItem(itemId)
  await scriptsWidget.value.setMusicItem(itemId)
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>

</style>
