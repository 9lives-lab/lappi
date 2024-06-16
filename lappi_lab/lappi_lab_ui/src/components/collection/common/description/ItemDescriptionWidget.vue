<template>
  <div class="item-decription-pane">
    <ItemDescriptionContent ref="contentPane"></ItemDescriptionContent>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import ItemDescriptionContent from 'src/components/collection/ItemDescriptionContent.vue'

const contentPane = ref(null)

onMounted(() => {
  contentPane.value.setText('Content')
})
</script>

<style lang="sass" scoped>

</style>
