<template>
  <div class="chat-pane column">
    <div class="col column justify-end">
      <q-scroll-area class="scroll-area column" :visible="true" ref="scrollArea">
        <div class="chat column justify-end q-pa-md q-gutter-md">
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
        class="col q-ma-md"
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
import { ref, nextTick } from 'vue'
import ChatMessage from 'src/components/chat/ChatMessage.vue'

const scrollArea = ref(null)
const message = ref('')
const chatMessages = ref([])

const sendMessage = async () => {
  console.log('Sending message:', message.value)
  chatMessages.value.push({ text: message.value, bot: false })
  chatMessages.value.push({ text: 'I am a bot', bot: true })
  message.value = ''

  await nextTick()

  const scrollTarget = scrollArea.value.getScrollTarget()
  scrollArea.value.setScrollPosition('vertical', scrollTarget.scrollHeight, 0)
}
</script>

<style lang="sass" scoped>
.chat-pane
  .scroll-area
    height: 100%
    max-width: 100%

.chat
  height: 100%

</style>
