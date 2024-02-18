<template>
  <q-page class="index-page row flex-center items-stretch">
    <ChatPane class="col-6"/>
  </q-page>
</template>

<script setup>
import ChatPane from 'src/components/chat/ChatPane.vue'

</script>

<style lang="sass" scoped>
.index-page
  background-image: url( '~assets/lappi_pattern.svg' )
  background-position: center
  background-size: 350px
  background-repeat: no-repeat

</style>
