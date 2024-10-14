<template>
  <WidgetPane :title="message.role === 'Assistant' ? 'Lappi:' : 'User:'" class="message">
    <div class="message-text q-pa-md" v-html="formatMessageContent(message.content)"></div>
  </WidgetPane>
</template>

<script setup>
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'

defineProps({
  message: Object
})

const formatMessageContent = (content) => {
  return content.replace(/\n/g, '<br>')
}
</script>

<style lang="sass" scoped>

</style>
