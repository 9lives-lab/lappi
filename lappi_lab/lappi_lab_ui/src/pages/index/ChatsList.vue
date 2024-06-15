<template>
  <WidgetPane title="Chats" class="column">
    <ToolPane>
      <q-btn label="New Chat" icon="add" @click="openChat"></q-btn>
    </ToolPane>
  </WidgetPane>
</template>

<script setup>
import { getCurrentInstance } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'

const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi
const emit = defineEmits(['openChat'])

const openChat = () => {
  aminaApi.sendRequest('lappi.chat.create_chat')
  emit('openChat')
}

</script>

<style lang="sass" scoped>

</style>
