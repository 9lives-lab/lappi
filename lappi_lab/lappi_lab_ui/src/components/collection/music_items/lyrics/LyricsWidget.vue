<template>
  <div class="column lyrics-widget">
    <ToolPane class="col">
      <q-btn class="col-auto" icon="save" label="Save" @click="saveLyrics" />
    </ToolPane>
    <q-input class="lyrics-editor" v-model="text" borderless autogrow />
  </div>
</template>

<script setup>
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

defineExpose({
  updateItem
})
</script>

<style lang="sass">
.lyrics-editor
  textarea
    text-align: center !important

</style>
