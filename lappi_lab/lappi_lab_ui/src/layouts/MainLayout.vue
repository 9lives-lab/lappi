<template>
  <q-layout view="hHh Lpr lFf">
    <q-header bordered class="header">
      <div class="row items-center">
        <div class="col-auto text-h6 text-weight-light q-ml-lg q-mr-md">
          Lappi Lab
        </div>
        <PlayerPane class="col"></PlayerPane>
      </div>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
      mini
    >
      <q-list>
        <q-item-label
          header
        >
          Menu
        </q-item-label>

        <RouterIcon
          v-for="link in linksList"
          :key="link.title"
          v-bind="link"
        />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
import { defineComponent, ref } from 'vue'
import RouterIcon from 'src/amina_ui/components/RouterIcon.vue'
import PlayerPane from 'src/components/player/PlayerPane.vue'

const linksList = [
  {
    title: 'Collection',
    caption: '',
    icon: 'diamond',
    color_style: 'color: #e06060;',
    link: '/collection_tree'
  },
  {
    title: 'Playlists',
    caption: '',
    icon: 'headphones',
    color_style: 'color: #72ba85;',
    link: '/playlists'
  },
  {
    title: 'FileManager',
    caption: '',
    icon: 'folder',
    color_style: 'color: #60bae0;',
    link: '/file_manager'
  },
  {
    title: 'Settings',
    caption: '',
    icon: 'settings',
    color_style: 'color: #a8e6df;',
    link: '/settings'
  },
  {
    title: 'Commands',
    caption: '',
    icon: 'terminal',
    color_style: 'color: #83d4c1;',
    link: '/commands'
  }
]

export default defineComponent({
  name: 'MainLayout',

  components: {
    RouterIcon,
    PlayerPane
  },

  setup () {
    const leftDrawerOpen = ref(false)

    return {
      linksList,
      leftDrawerOpen,
      toggleLeftDrawer () {
        leftDrawerOpen.value = !leftDrawerOpen.value
      }
    }
  }
})
</script>

<style lang="sass" scoped>
.header
  background-color: $amina-title

</style>
