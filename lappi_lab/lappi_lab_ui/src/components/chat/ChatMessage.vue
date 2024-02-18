<template>
  <AminaPlate class="message column">
    <TitledPane :title="message.bot ? 'Lappi spirit:' : 'You:'" class="message-header">
      <div class="message-text q-pa-md">
        {{ message.text }}
      </div>
    </TitledPane>
  </AminaPlate>
</template>

<script setup>
import AminaPlate from 'src/amina_ui/components/UiPlate.vue'
import TitledPane from 'src/amina_ui/components/TitledPane.vue'

defineProps({
  message: Object
})
</script>

<style lang="sass" scoped>

</style>
