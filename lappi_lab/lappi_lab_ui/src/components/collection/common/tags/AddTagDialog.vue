<template>
  <div class="flex">
    <q-btn class="add-button" icon="add" label="Add Tag" @click="dialogVisible = true" />
    <q-dialog v-model="dialogVisible">
      <q-card>
        <q-card-section>
          <div class="q-pb-md">Add a new tag</div>
          <div class="row items-center">
            <q-input square filled dense placeholder="Tag name" v-model="newTagName" />
            <q-select
              v-model="selectedTagType"
              :options="tagTypes"
              label="Type"
              square filled dense
            />
          </div>
        </q-card-section>
        <q-card-actions>
          <q-btn label="Add" @click="addTag" />
          <q-btn label="Close" @click="dialogVisible = false" />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps({
  adapter: {
    type: Object,
    required: true
  },
})

const newTagName = ref('')
const dialogVisible = ref(false)
const selectedTagType = ref('String')
const tagTypes = ref(['String', 'Number'])

async function addTag() {
  let value = { }
  if (selectedTagType.value === 'String') {
    value.String = ''
  } else if (selectedTagType.value === 'Number') {
    value.Number = 0
  }

  await props.adapter.setTag(newTagName.value, value)
  newTagName.value = ''
  dialogVisible.value = false
}
</script>

<style lang="sass" scoped>
.add-button
  color: $amina-positive

</style>
