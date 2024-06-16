<template>
  <div class="main-chat row q-pa-md">
    <div class="col-3"/>
    <ChatPane class="col-6" ref="chatPane"/>
    <ChatsList class="col-3" @open-chat="onOpenChat"/>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import ChatsList from 'src/components/chat/ChatsList.vue'
import ChatPane from 'src/components/chat/ChatPane.vue'

const chatPane = ref(null)

const onOpenChat = () => {
  chatPane.value.openChat()
}

</script>

<style lang="sass" scoped>
.main-chat
  background-image: url( '~assets/lappi_pattern.svg' )
  background-position: center
  background-size: 350px
  background-repeat: no-repeat

</style>
