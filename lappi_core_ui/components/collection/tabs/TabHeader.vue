<template>
  <div class="editor-header text-subtitle1 col q-pa-lg items-center q-gutter-sm">
    <div class="text-weight-medium">{{ title }}</div>
  </div>
</template>

<script setup>

defineProps({
  itemType: String,
  title: String
})

</script>

<style lang="sass" scoped>
.editor-header
  text-align: center
  text-shadow: 0px 0px 16px rgba(255,255,255,0.4)

</style>
