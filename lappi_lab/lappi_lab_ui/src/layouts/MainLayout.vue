<template>
  <q-layout class="main-layout text-weight-light" view="hHh Lpr lFf">
    <q-header class="header">
      <div class="row items-center">
        <div class="col-auto text-h6 text-weight-light q-ml-lg q-mr-md">
          <router-link class="root-link" to="/">Lappi Lab</router-link>
        </div>
        <PlayerPane class="col"></PlayerPane>
      </div>
    </q-header>

    <q-drawer v-model="leftDrawerOpen" show-if-above mini >
      <q-list>
        <q-item-label header>Menu</q-item-label>
        <RouterIcon v-for="link in linksList" :key="link.title" v-bind="link" />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>

    <q-footer class="footer q-pa-sm"></q-footer>
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
  background-color: $amina-primary-color

  .root-link
    color: white
    text-decoration: none

.footer
  background-color: transparent

</style>
