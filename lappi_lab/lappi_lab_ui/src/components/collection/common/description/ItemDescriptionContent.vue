<template>
  <div class="item-description-content">
    {{ text }}
  </div>
</template>

<script setup>
import { ref } from 'vue'

const text = ref('')

function setText (newText) {
  text.value = newText
}

function clear () {
  text.value = ''
}

defineExpose({
  setText,
  clear
})
</script>

<style lang="sass" scoped>
.item-description-content
  padding: $amina-default-padding

</style>
