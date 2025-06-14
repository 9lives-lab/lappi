<template>
  <div class="item-editor col q-gutter-md">
    <TabHeader itemType="" :title="itemName" />
    <TagsWidget ref="tagsWidget"/>
    <ChatExploringWidget ref="chatExploringWidget" />
    <LyricsWidget ref="lyricsWidget" />
    <AddToPlaylistWidget ref="addToPlaylistWidget" />
    <MusicFileWidget ref="musicFileWidget" />
    <MusicLinksWidget ref="musicLinksWidget" />
    <MusicItemSummaryWidget ref="musicItemSummaryWidget" />
    <ScriptsWidget ref="scriptsWidget" />
  </div>
</template>

<script setup>
import { ref, getCurrentInstance } from 'vue'
import TabHeader from 'src/components/collection/tabs/TabHeader.vue'
import TagsWidget from 'src/components/collection/common/tags/TagsWidget.vue'
import MusicItemSummaryWidget from 'src/components/collection/tabs/music_item/MusicItemSummaryWidget.vue'
import MusicFileWidget from 'src/components/collection/tabs/music_item/MusicFileWidget.vue'
import MusicLinksWidget from 'src/components/collection/tabs/music_item/MusicLinksWidget.vue'
import LyricsWidget from 'src/components/collection/tabs/music_item/lyrics/LyricsWidget.vue'
import AddToPlaylistWidget from 'src/components/collection/common/AddToPlaylistWidget.vue'
import ChatExploringWidget from 'src/components/collection/common/exploring/ChatExploringWidget.vue'
import ScriptsWidget from 'src/components/collection/common/ScriptsWidget.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const tagsWidget = ref(null)
const musicItemSummaryWidget = ref(null)
const lyricsWidget = ref(null)
const addToPlaylistWidget = ref(null)
const musicFileWidget = ref(null)
const musicLinksWidget = ref(null)
const chatExploringWidget = ref(null)
const scriptsWidget = ref(null)

const itemName = ref('null')

async function updateItem (itemId) {
  const itemDescription = await aminaApi.sendRequest('lappi.collection.music.get_item_description', { item_id: itemId })
  itemName.value = itemDescription.name

  await tagsWidget.value.setMusicItem(itemId)
  await musicItemSummaryWidget.value.update(itemId)
  await lyricsWidget.value.updateItem(itemId)
  await addToPlaylistWidget.value.setMusicItem(itemId)
  await chatExploringWidget.value.update({ MusicItem: itemId })
  await musicFileWidget.value.updateItem(itemId)
  await musicLinksWidget.value.updateItem(itemId)
  await scriptsWidget.value.setMusicItem(itemId)
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>

</style>
