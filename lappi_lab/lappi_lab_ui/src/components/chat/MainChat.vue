<template>
  <div class="main-chat row q-pa-md">
    <div class="col-3"/>
    <ChatPane class="col-6" ref="chatPane"/>
    <div class="col-3"/>
  </div>
</template>

<script setup>
import ChatPane from 'src/components/chat/ChatPane.vue'

</script>

<style lang="sass" scoped>
.main-chat
  background-image: url( '~assets/lappi_pattern.svg' )
  background-position: center
  background-size: 350px
  background-repeat: no-repeat

</style>
