<template>
  <div class="player-pane row items-center items-stretch">
    <q-btn class="col-auto text-subtitle2" icon="skip_previous" />
    <q-btn class="col-auto text-subtitle1" :icon="playButtonIcon" @click="tooglePlay()"/>
    <q-btn class="col-auto text-subtitle2" icon="skip_next" />
    <div class="col q-pr-md q-pl-md q-pt-md">
      <div class="text-weight-medium">{{ title }}</div>
      <q-slider
        class="col"
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
const playButtonIcon = ref('play_arrow')
const progress = ref(0)

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
      playButtonIcon.value = 'pause'
    } else {
      playButtonIcon.value = 'play_arrow'
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

</style>
