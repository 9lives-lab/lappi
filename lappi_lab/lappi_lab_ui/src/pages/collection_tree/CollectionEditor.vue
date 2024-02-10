<template>
  <div class="">
    <ArtistEditor
      ref="artistEditor"
      v-show="editorType === 'artist'"
    />
    <ItemEditor
      ref="itemEditor"
      v-show="editorType === 'item'"
    />
    <div v-show="editorType === 'none'">None</div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import ArtistEditor from 'src/components/collection/artists/ArtistEditor.vue'
import ItemEditor from 'src/components/collection/music_items/ItemEditor.vue'

const artistEditor = ref(null)
const itemEditor = ref(null)
const editorType = ref('none')

async function setArtist (artistId) {
  editorType.value = 'artist'
  await artistEditor.value.updateArtist(artistId)
}

async function setItem (itemId) {
  editorType.value = 'item'
  await itemEditor.value.updateItem(itemId)
}

defineExpose({
  setArtist,
  setItem
})
</script>

<style lang="sass" scoped>

</style>
