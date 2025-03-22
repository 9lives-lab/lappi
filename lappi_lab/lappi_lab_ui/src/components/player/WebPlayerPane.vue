<template>
  <div></div>
</template>

<script setup>
import { getCurrentInstance, onMounted, onUnmounted } from 'vue'
import {Howl} from 'howler'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

let webPlayer = null
let webPlayerTimerId = 0

function startWebPlayer(fileName) {
  if (webPlayer !== null) {
    webPlayer.stop()
  }

  const srcPath = aminaApi.getFileUrl('lappi.palyback.web/' + fileName)
  webPlayer = new Howl({
    src: [srcPath],
    html5: true,
    onend: async () => {
      await aminaApi.sendRequest('lappi.playback.web.on_web_player_state_changed', { web_state: 'PlaybackFinished', progress: 0 })
    },
  })

  webPlayer.play()

  if (webPlayerTimerId === 0) {
    webPlayerTimerId = setInterval(async () => {
      await updateWebPlayer()
    }, 200);
  }
}

async function stopWebPlayer() {
  if (webPlayerTimerId !== 0) {
    clearInterval(webPlayerTimerId)
  }
  if (webPlayer !== null) {
    webPlayer.stop()
  }
  await aminaApi.sendRequest('lappi.playback.web.on_web_player_state_changed', { web_state: 'Stopped', progress: 0.0 })
}

function getWebPlayerProgress() {
  if (webPlayer !== null) {
    return webPlayer.seek() / webPlayer.duration()
  }
  return 0
}

async function updateWebPlayer() {
  if (webPlayer !== null) {
    if (webPlayer.playing()) {
      await aminaApi.sendRequest('lappi.playback.web.on_web_player_state_changed', { web_state: 'Playing', progress: getWebPlayerProgress() })
    }
  }
}

onMounted(async () => {  
  aminaApi.setEventHandler('lappi.playback.web.OnWebPlayerCommand', 'WebPlayerPane', async (event) => {
    if (event.command.type === 'Play') {
      startWebPlayer(event.command.file_name)
    } else if (event.command.type === 'Pause') {
      webPlayer.pause()
      const progress = getWebPlayerProgress()
      await aminaApi.sendRequest('lappi.playback.web.on_web_player_state_changed', { web_state: 'Paused', progress })
    } else if (event.command.type === 'Resume') {
      webPlayer.play()
      await aminaApi.sendRequest('lappi.playback.web.on_web_player_state_changed', { web_state: 'Playing', progress: getWebPlayerProgress() })
    } else if (event.command.type === 'Seek') {
      webPlayer.seek(webPlayer.duration() * event.command.progress)
    } else if (event.command.type === 'Stop') {
      webPlayer.stop()
      await aminaApi.sendRequest('lappi.playback.web.on_web_player_state_changed', { web_state: 'Stopped', progress: 0.0 })
    }
  })

  await stopWebPlayer()
})

onUnmounted(async () => {
  aminaApi.removeEventHandler('lappi.playback.web.OnWebPlayerCommand', 'WebPlayerPane')
  await stopWebPlayer()
})
</script>
