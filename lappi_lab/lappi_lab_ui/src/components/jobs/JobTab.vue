<template>
  <div class="column q-gutter-md">
    <WidgetPane :title="jobName" class="col-auto">
      <div class="column q-pa-md q-gutter-md">
        <div class="">{{ jobDescription }}</div>
        <div class="row">
          <q-badge outline color="primary" size="md" :label="stageLabel" />
          <div class="q-ml-md">{{ stateText }}</div>
        </div>
      </div>
      <ToolPane class="controls-pane">
        <q-btn class="start-button" label="Start" icon="play_arrow" @click="startJob" />
        <q-btn class="stop-button" label="Stop" icon="stop" @click="stopJob" />
      </ToolPane>
      <q-linear-progress stripe size="10px" animation-speed="0" :value="progress" />
    </WidgetPane>
    <div class="col"></div>
  </div>
</template>

<script setup>
import { onMounted, onUnmounted, getCurrentInstance, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import ToolPane from 'src/amina_ui/components/ToolPane.vue'

const lappiApi = getCurrentInstance().appContext.config.globalProperties.$lappiApi

const jobName = ref('')
const jobDescription = ref('')
const stageLabel = ref('')
const stateText = ref('')
const progress = ref(50)

let currentJobId = ''

async function startJob () {
  await lappiApi.sendRequest('lappi.jobs.start_job', { job_id: currentJobId })
}

async function stopJob () {
  await lappiApi.sendRequest('lappi.jobs.stop_job', { job_id: currentJobId })
}

async function update (fullUpdate) {
  if (fullUpdate) {
    const description = await lappiApi.sendRequest('lappi.jobs.get_job_description', { job_id: currentJobId })
    jobName.value = description.name
    jobDescription.value = description.description
  }

  const state = await lappiApi.sendRequest('lappi.jobs.get_job_state', { job_id: currentJobId })
  stateText.value = state.state_text
  progress.value = state.progress
  stageLabel.value = state.stage
}

async function updateJob (newJobId) {
  currentJobId = newJobId
  await update(true)
}

onMounted(() => {
  lappiApi.setEventHandler('lappi.jobs.OnJobStateChanged', 'JobTab', () => {
    console.log('OnJobStateChanged')
    update(false)
  })
})

onUnmounted(() => {
  lappiApi.removeEventHandler('lappi.jobs.OnJobStateChanged', 'JobTab')
})

defineExpose({
  updateJob
})
</script>

<style lang="sass" scoped>
.start-button
  color: $amina-positive
.stop-button
  color: $amina-negative

</style>

