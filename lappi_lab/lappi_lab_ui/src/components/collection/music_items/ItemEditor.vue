<template>
  <div class="item-editor">
    <TitledPane title="Artists">
      <ArtistList ref="artistList" class="q-ma-sm"/>
    </TitledPane>
    <TitledPane title="Tags">
      <TagsEditor ref="tagsEditor" class="q-ma-sm"/>
    </TitledPane>
    <TitledPane title="Search description">
      <ItemDescriptionPane />
    </TitledPane>
    <TitledPane title="Play">
      <PlaybackSources ref="playbackSources" class="col-auto" />
    </TitledPane>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import TitledPane from 'src/amina_ui/components/TitledPane.vue'
import ArtistList from 'src/components/collection/music_items/ArtistList.vue'
import TagsEditor from 'src/components/collection/music_items/TagsEditor.vue'
import PlaybackSources from 'src/components/collection/music_items/PlaybackSources.vue'
import ItemDescriptionPane from 'src/components/collection/music_items/ItemDescriptionPane.vue'

const artistList = ref(null)
const tagsEditor = ref(null)
const playbackSources = ref(null)

async function updateItem (itemId) {
  await artistList.value.updateItem(itemId)
  await tagsEditor.value.updateItem(itemId)
  await playbackSources.value.updateItem(itemId)
}

defineExpose({
  updateItem
})
</script>

<style lang="sass" scoped>

</style>
