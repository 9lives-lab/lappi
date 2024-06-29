<template>
  <WidgetPane title="Lyrics">
    <div class="column lyrics-widget">
      <ToolPane class="col">
        <q-btn class="save-button col-auto" icon="save" label="Save" @click="saveLyrics" />
        <q-btn class="undo-button col-auto" icon="undo" label="Undo" @click="undoChanges" />
        <q-btn class="find-button col-auto" icon="lyrics" label="Find lyrics" @click="findLyrics" />
      </ToolPane>
      <q-input class="lyrics-editor" v-model="text" borderless autogrow />
    </div>
  </WidgetPane>
</template>

<script setup>
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'
import { ref, getCurrentInstance } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const text = ref('')

let currentMusicItemId = -1
let currentLyricsId = -1

async function updateItem (itemId) {
  currentMusicItemId = itemId
  const lyricsList = await aminaApi.sendRequest('lappi.collection.lyrics.get_lyrics_list', { music_id: currentMusicItemId })
  if (lyricsList.length > 0) {
    currentLyricsId = lyricsList[0].lyrics_id
    const lyricsText = await aminaApi.sendRequest('lappi.collection.lyrics.get_lyrics', { lyrics_id: currentLyricsId })
    text.value = lyricsText
  } else {
    currentLyricsId = -1
    text.value = ''
  }
}

async function saveLyrics () {
  if (currentMusicItemId >= 0) {
    if (currentLyricsId < 0) {
      currentLyricsId = await aminaApi.sendRequest('lappi.collection.lyrics.add_lyrics_item', { music_item_id: currentMusicItemId, lang_code: 'en' })
    }
    await aminaApi.sendRequest('lappi.collection.lyrics.save_lyrics', { lyrics_id: currentLyricsId, text: text.value })
  }
}

async function undoChanges () {
  await updateItem(currentMusicItemId)
}

async function findLyrics () {
  try {
    text.value = await aminaApi.sendRequest('lappi.exploring.lyrics.find_lyrics', { music_item_id: currentMusicItemId })
  } catch (err) {
    text.value = 'Error: ' + err
  }
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>
.save-button
  color: $amina-positive
.undo-button
  color: $amina-negative
.find-button
  color: $amina-info
</style>

<style lang="sass">
.lyrics-editor
  textarea
    text-align: center !important

</style>
