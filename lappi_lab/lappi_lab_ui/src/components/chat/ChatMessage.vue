<template>
  <WidgetPane :title="message.role === 'Assistant' ? 'Lappi spirit:' : 'You:'" class="message">
    <div class="message-text q-pa-md">{{ message.content }}</div>
  </WidgetPane>
</template>

<script setup>
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'

defineProps({
  message: Object
})
</script>

<style lang="sass" scoped>

</style>
