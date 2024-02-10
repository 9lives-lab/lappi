<template>
  <div class="row">
    <q-input
      dense
      borderless
      class="text-field col"
      v-model="titleModel"
    />
  </div>
</template>

<script setup>
import { ref } from 'vue'

const titleModel = ref('')

defineProps({
  title: String
})

</script>

<style lang="sass" scoped>

</style>
