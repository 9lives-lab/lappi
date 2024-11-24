<template>
  <div class="player-pane row items-center items-stretch">
    <q-btn class="col-auto text-subtitle2" icon="skip_previous" @click="playPrevious" />
    <q-btn class="col-auto text-subtitle1" :icon="playButtonIcon" size="lg" @click="tooglePlay()"/>
    <q-btn class="col-auto text-subtitle2" icon="skip_next" @click="playNext"/>
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
        @change="onProgressChange"
      />
    </div>
  </div>
</template>

<script setup>
import { getCurrentInstance, onMounted, ref } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const title = ref(' ')
const playButtonIcon = ref('play_circle')
const progress = ref(0)

async function playPrevious () {
  await aminaApi.sendRequest('lappi.playback.play_previous')
}

async function playNext () {
  await aminaApi.sendRequest('lappi.playback.play_next')
}

async function tooglePlay () {
  await aminaApi.sendRequest('lappi.playback.toggle')
}

async function onProgressChange (value) {
  await aminaApi.sendRequest('lappi.playback.seek', { progress: value })
}

onMounted(() => {
  aminaApi.setEventHandler('lappi.playback.OnStateUpdated', (event) => {
    title.value = event.title
    progress.value = event.progress
    if (event.is_playing) {
      playButtonIcon.value = 'pause_circle'
    } else {
      playButtonIcon.value = 'play_circle'
    }
  })
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

</style>

<style lang="sass">
.player-pane
  .q-icon
    text-shadow: 0px 0px 10px rgba(255,255,255,0.2)

  .progress-slider
    .q-slider__track-container
      cursor: default

</style>
