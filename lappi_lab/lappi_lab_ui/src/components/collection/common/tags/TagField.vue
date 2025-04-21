<template>
  <div class="tag-field row">
    <div class="tag-span col-auto">
      {{ name }}
    </div>
    <q-input
      dense
      borderless
      :disable="readonly"
      class="text-field col"
      v-model="model"
      :rules="[validateValue]"
      @update:model-value="updateTag"
    >
      <template v-slot:append>
        <q-icon :name="valueTypeIcon" />
      </template>
    </q-input>
    <div 
      v-show="!readonly"
      class="icons-bar col-auto"
      @click="confirmDelete = true"
    >
      <q-icon name="delete" />

      <q-dialog v-model="confirmDelete" persistent>
        <q-card>
          <q-card-section class="row items-center">
            <span class="q-ml-sm">Are you sure you want to delete tag '{{ name }}'?</span>
          </q-card-section>

          <q-card-actions align="right">
            <q-btn flat label="Cancel" color="primary" v-close-popup />
            <q-btn flat label="Delete" color="primary" v-close-popup @click="deleteTag" />
          </q-card-actions>
        </q-card>
      </q-dialog>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

let adapter = null

const props = defineProps({
  readonly: Boolean,
  tagAdapter: Object
})

const name = ref('')
const valueType = ref('String')
const valueTypeIcon = ref('')
const model = ref('')

const confirmDelete = ref(false)

async function deleteTag () {
  adapter.deleteTag()
}

function validateValue (value) {
  if (valueType.value === 'Number') {
    const num = Number(value)
    if (isNaN(num)) {
      return 'Value must be a number'
    }
  }
  return true
}

async function updateTag (newValue) {
  if (valueType.value === 'String') {
    await props.tagAdapter.setValue({ String: newValue })
  } else if (valueType.value === 'Number') {
    const num = Number(newValue)
    if (validateValue(newValue) !== true) {
      return
    }
    await props.tagAdapter.setValue({ Number: num })
  }
}

watch(() => props.tagAdapter, async (newAdapter) => {
  if (newAdapter) {
    adapter = newAdapter
    name.value = newAdapter.tagName
    if (newAdapter.initialValue.String) {
      valueType.value = 'String'
      valueTypeIcon.value = 'text_fields'
      model.value = newAdapter.initialValue.String
    } else if (newAdapter.initialValue.Number) {
      valueType.value = 'Number'
      valueTypeIcon.value = '123'
      model.value = newAdapter.initialValue.Number.toString()
    } else {
      valueType.value = 'Boolean'
      valueTypeIcon.value = 'check_box'
      model.value = 'True'
    }
  }
}, { immediate: true })
</script>

<style lang="sass" scoped>
.tag-field
  min-width: 200px
  max-width: 600px
  background-color: $primary
  border-color: $amina-separator-color
  border-width: 1px
  border-style: solid
  border-radius: 5px
  box-shadow: rgba(0, 0, 0, 0.14) 0px 0px 12px
  &:hover
    box-shadow: rgba(0.2, 0.2, 0.2, 0.24) 0px 0px 10px

  .tag-span
    display: flex
    justify-content: center
    align-items: center
    padding: 0 10px 0 10px
    border-color: $amina-separator-color
    border-width: 0 1px 0 0
    border-style: solid

  .text-field
    height: 35px
    background-color: $amina-background-primary-color
    padding: 0 10px 0 10px

  .icons-bar
    display: flex
    justify-content: center
    align-items: center
    padding: 0 10px 0 10px
    border-color: $amina-separator-color
    border-radius: 0 5px 5px 0
    border-width: 0 0 0 1px
    border-style: solid
    font-size: 1.5em
    &:hover
      background-color: $secondary

</style>
