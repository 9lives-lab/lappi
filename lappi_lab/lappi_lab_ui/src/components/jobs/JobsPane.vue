<template>
  <div class="row q-pa-md">
    <WidgetPane class="col-3 q-mr-md">
      <AbsoluteWrapper class="list-wrapper col">
        <q-virtual-scroll class="list-scroll" :items="jobsList" v-slot="{ item }">
          <q-item class="list-item" :key="item.jobId" clickable @click="selectJob(item.jobId)">
            <q-item-section avatar>
              <q-icon :name="item.icon" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ item.name }}</q-item-label>
              <q-item-label caption>{{item.stageLabel}}</q-item-label>
            </q-item-section>
          </q-item>
        </q-virtual-scroll>
      </AbsoluteWrapper>
    </WidgetPane>
    <JobTab class="col" ref="jobTab" />
  </div>
</template>

<script setup>
import { getCurrentInstance, onMounted, onUnmounted, ref } from 'vue'
import WidgetPane from 'src/amina_ui/components/WidgetPane.vue'
import AbsoluteWrapper from 'src/amina_ui/components/AbsoluteWrapper.vue'
import JobTab from 'src/components/jobs/JobTab.vue'

const lappiApi = getCurrentInstance().appContext.config.globalProperties.$aminaApi

const jobTab = ref(null)

const jobsList = ref([])
const selectedJobId = ref('')

async function selectJob (newJobId) {
  selectedJobId.value = newJobId
  await jobTab.value.updateJob(newJobId)
}

async function update () {
  const newJobsList = []

  for (const jobDesc of await lappiApi.sendRequest('lappi.jobs.get_available_jobs')) {
    const item = {
      jobId: jobDesc.job_id,
      name: jobDesc.name,
      icon: jobDesc.icon,
      description: jobDesc.description,
      stageLabel: ''
    }

    newJobsList.push(item)
  }

  jobsList.value = newJobsList

  if (selectedJobId.value === '') {
    await selectJob(newJobsList[0].jobId)
  }

  await updateState()
}

async function updateState () {
  for (let i = 0; i < jobsList.value.length; i++) {
    const currentJobId = jobsList.value[i].jobId
    const state = await lappiApi.sendRequest('lappi.jobs.get_job_state', { job_id: currentJobId })
    jobsList.value[i].stageLabel = `[${state.stage}] ${state.state_text}`
  }
}

onMounted(() => {
  lappiApi.setEventHandler('lappi.jobs.OnJobStateChanged', 'JobPane', () => {
    updateState()
  })

  update()
})

onUnmounted(() => {
  lappiApi.removeEventHandler('lappi.jobs.OnJobStateChanged', 'JobPane')
})

</script>

<style lang="sass" scoped>

</style>
