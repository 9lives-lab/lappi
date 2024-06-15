<template>
  <div class="item-editor col q-gutter-md">
    <EditorHeader itemType="" :title="itemName" />
    <WidgetPane title="Tags">
      <TagsEditor ref="tagsEditor" class="q-ma-sm"/>
    </WidgetPane>
    <WidgetPane title="Lyrics">
      <LyricsWidget ref="lyricsWidget" />
    </WidgetPane>
    <WidgetPane title="Play">
      <PlaybackSources ref="playbackSources" class="col-auto" />
    </WidgetPane>
  </div>
</template>

<script setup>
import { ref, getCurrentInstance } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import EditorHeader from 'src/components/collection/EditorHeader.vue'
import TagsEditor from 'src/components/collection/music_items/TagsEditor.vue'
import PlaybackSources from 'src/components/collection/music_items/PlaybackSources.vue'
import LyricsWidget from 'src/components/collection/music_items/lyrics/LyricsWidget.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const tagsEditor = ref(null)
const lyricsWidget = ref(null)
const playbackSources = ref(null)

const itemName = ref('null')

async function updateItem (itemId) {
  const itemDescription = await aminaApi.sendRequest('lappi.collection.music.get_item_description', { item_id: itemId })
  itemName.value = itemDescription.name

  await tagsEditor.value.updateItem(itemId)
  await lyricsWidget.value.updateItem(itemId)
  await playbackSources.value.updateItem(itemId)
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>

</style>
