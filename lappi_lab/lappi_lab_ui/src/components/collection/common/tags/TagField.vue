<template>
  <div class="tag-field row">
    <div class="tag-span col-auto">
      {{ name }}
    </div>
    <q-input
      dense
      borderless
      class="text-field col"
      v-model="model"
    />
    <div class="icons-bar col-auto">
      <q-icon name="delete" @clic="deleteTag" />
    </div>
  </div>
</template>

<script setup>
import { defineModel } from 'vue'

defineProps({
  name: String
})

const model = defineModel()
const emit = defineEmits(['deleteTag'])

async function deleteTag () {
  emit('deleteTag')
}
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
