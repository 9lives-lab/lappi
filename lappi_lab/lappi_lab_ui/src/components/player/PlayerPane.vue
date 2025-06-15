<template>
  <div class="player-pane row items-stretch">
    <q-btn class="col-auto text-subtitle2" icon="skip_previous" @click="playPrevious" />
    <q-btn class="col-auto text-subtitle1" :icon="playButtonIcon" size="lg" @click="tooglePlay()"/>
    <q-btn class="col-auto text-subtitle2" icon="skip_next" @click="playNext"/>
    <div class="cover-image-container row items-center q-pl-md">
      <img v-show="coverUrl != null" :src="coverUrl" class="cover-image">
    </div>
    <div class="col q-pr-md q-pl-md q-pt-md">
      <div class="title text-weight-light">{{ title }}</div>
      <q-slider
        class="col progress-slider"
        v-model="progress"
        :min="0"
        :max="1000"
        color="light-blue-5"
        track-color="dark"
        inner-track-color="transparent"
        track-size="6px"
        thumb-size="0px"
        @update:model-value="onProgressChange"
      />
    </div>
    <WebPlayerPane/>
  </div>
</template>

<script setup>
import { getCurrentInstance, onMounted, onUnmounted, ref } from 'vue'
import WebPlayerPane from 'src/components/player/WebPlayerPane.vue'

const lappiApi = getCurrentInstance().appContext.config.globalProperties.$lappiApi

const title = ref(' ')
const coverUrl = ref(null)
const playButtonIcon = ref('play_circle')
const progress = ref(0)
let isProgressChanged = false

async function playPrevious () {
  await lappiApi.sendRequest('lappi.playback.play_previous')
}

async function playNext () {
  await lappiApi.sendRequest('lappi.playback.play_next')
}

async function tooglePlay () {
  await lappiApi.sendRequest('lappi.playback.toggle')
}

async function onProgressChange (value) {
  isProgressChanged = true
  await lappiApi.sendRequest('lappi.playback.seek', { progress: value })
}

onMounted(() => {
  lappiApi.setEventHandler('lappi.playback.OnStateUpdated', 'PlayerPane', async (event) => {
    if (isProgressChanged === false) {
      title.value = event.title
      progress.value = event.progress

      if (event.is_playing) {
        playButtonIcon.value = 'pause_circle'
      } else {
        playButtonIcon.value = 'play_circle'
      }

      if (event.cover_picture !== null) {
        coverUrl.value = await lappiApi.getPictureUrl(event.cover_picture)
      } else {
        coverUrl.value = null
      }
    } else {
      // skip progress change event
      isProgressChanged = false
    }
  })
})

onUnmounted(() => {
  lappiApi.removeEventHandler('lappi.playback.OnStateUpdated', 'PlayerPane')
})
</script>

<style lang="sass" scoped>
.player-pane
  .q-btn
    border-radius: 0px

  .q-btn::before
    box-shadow: none

  .title
    text-align: center

  .cover-image
    height: 50px
    border-radius: 5px
    border: 2px solid rgba(255,255,255,0.1)
    box-shadow: 0px 0px 10px rgba(255,255,255,0.1)

</style>

<style lang="sass">
.player-pane
  .q-icon
    text-shadow: 0px 0px 10px rgba(255,255,255,0.2)

  .progress-slider
    .q-slider__track-container
      cursor: default

</style>
