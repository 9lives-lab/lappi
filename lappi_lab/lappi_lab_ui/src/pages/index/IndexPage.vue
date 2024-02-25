<template>
  <q-page class="index-page row q-pa-md">
    <div class="col-3"/>
    <ChatPane class="col-6" ref="chatPane"/>
    <ChatsList class="col-3" @open-chat="onOpenChat"/>
  </q-page>
</template>

<script setup>
import { ref } from 'vue'
import ChatsList from 'src/pages/index/ChatsList.vue'
import ChatPane from 'src/components/chat/ChatPane.vue'

const chatPane = ref(null)

const onOpenChat = () => {
  chatPane.value.openChat()
}

</script>

<style lang="sass" scoped>
.index-page
  background-image: url( '~assets/lappi_pattern.svg' )
  background-position: center
  background-size: 350px
  background-repeat: no-repeat

</style>
