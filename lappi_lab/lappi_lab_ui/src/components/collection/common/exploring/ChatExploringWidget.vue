<template>
  <WidgetPane title="Lappi spirit">
    <div class="column">
      <q-linear-progress :query="isInProgress" />
      <div class="row q-pa-md q-gutter-md">
        <q-btn
          v-for="item in templatesList"
          :key="item.id"
          :label="item.name"
          color="primary"
          no-caps
          @click="createChat(item.id)"
        />
      </div>
    </div>
  </WidgetPane>
</template>

<script setup>
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import { ref, getCurrentInstance } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const aminaApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const templatesList = ref([])
let templateContext = null
const isInProgress = ref(false)

async function update (newTemplateContext) {
  templateContext = newTemplateContext
  const newTemplatesList = await aminaApi.sendRequest('lappi.chat.templates.get_templates_list', { context: newTemplateContext })
  templatesList.value = newTemplatesList
  isInProgress.value = false
}

async function createChat (templateId) {
  isInProgress.value = true
  await aminaApi.sendRequest('lappi.chat.templates.create_chat_from_template', { id: templateId, context: templateContext })
  isInProgress.value = false
  router.push('/')
}

defineExpose({
  update
})
</script>
