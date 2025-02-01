<template>
  <div class="players-switch row items-center q-pl-lg q-pr-lg">
    <q-icon name="speaker" class="q-mr-sm" />
    <div>{{currentPlayerName}}</div>
    <q-menu>
      <q-list style="min-width: 100px">
        <q-item
          v-for="player in playersLis"
          :key="player.id"
          clickable
          v-close-popup
          @click="switchPlayer(player.id)"
        >
          <q-item-section>{{ player.name  }}</q-item-section>
        </q-item>
      </q-list>
    </q-menu>
  </div>
</template>

<script setup>
import { getCurrentInstance, ref, onMounted, onUnmounted } from 'vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const currentPlayerName = ref('Current device')
const playersLis = ref([])

async function switchPlayer (playerId) {
  await aminaApi.sendRequest('lappi.playback.switch_player', { player_id: playerId })
}

onMounted(async () => {
  playersLis.value = await aminaApi.sendRequest('lappi.playback.get_players_list')
  aminaApi.setEventHandler('lappi.playback.OnStateUpdated', 'PlayersSwitch', (event) => {
    currentPlayerName.value = event.current_player_name
  })
})

onUnmounted(() => {
  aminaApi.removeEventHandler('lappi.playback.OnStateUpdated', 'PlayersSwitch')
})
</script>

<style lang="sass" scoped>
.players-switch
  &:hover
    cursor: pointer
    background-color: $primary

</style>
