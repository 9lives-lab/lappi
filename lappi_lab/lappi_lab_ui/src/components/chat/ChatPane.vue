<template>
  <div class="chat-pane column q-gutter-md q-pl-md q-pr-md">
    <div class="col column justify-end">
      <q-scroll-area class="scroll-area fit column" :visible="true" ref="scrollArea">
        <div class="chat column justify-end q-gutter-md">
          <ChatMessage
            v-for="(msg, i) in chatMessages"
            :key="i"
            :message="msg"
          />
        </div>
      </q-scroll-area>
    </div>
    <div class="chat-input row col-auto">
      <q-input
        class="col"
        v-model="message"
        placeholder="Ask Lappi spirit anything..."
        filled
        autogrow
        @keydown.enter.prevent="sendMessage"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, getCurrentInstance, nextTick, onMounted } from 'vue'
import ChatMessage from 'src/components/chat/ChatMessage.vue'

const scrollArea = ref(null)
const message = ref('')
const chatMessages = ref([])
const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const scrollDown = async () => {
  await nextTick()
  const scrollTarget = scrollArea.value.getScrollTarget()
  scrollArea.value.setScrollPosition('vertical', scrollTarget.scrollHeight, 0)
}

const sendMessage = async () => {
  const messageVale = message.value
  message.value = ''

  chatMessages.value.push({ role: 'User', content: messageVale })
  await scrollDown()

  chatMessages.value = await aminaApi.sendRequest('lappi.chat.send_message', { message: messageVale })
  await scrollDown()
}

const openChat = async () => {
  chatMessages.value = []
}

defineExpose({
  openChat
})

onMounted(async () => {
  chatMessages.value = await aminaApi.sendRequest('lappi.chat.get_dialog')
  await scrollDown()
})
</script>

<style lang="sass" scoped>
.chat-pane
  .scroll-area
    height: 100%
    max-width: 100%

.chat
  height: 100%

</style>
