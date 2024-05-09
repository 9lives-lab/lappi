<template>
  <div class="editor-header col q-pa-lg items-center q-gutter-sm">
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

</style>
