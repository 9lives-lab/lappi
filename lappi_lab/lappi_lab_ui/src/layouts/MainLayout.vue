<template>
  <q-layout class="main-layout text-weight-light" view="hHh Lpr fFf">
    <q-header class="header">
      <div class="row items-center">
        <div class="col-auto text-h4 text-weight-light q-ml-xl q-mr-lg">
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

    <q-footer class="footer">
      <FooterToolbar></FooterToolbar>
    </q-footer>
  </q-layout>
</template>

<script>
import { defineComponent, ref } from 'vue'
import RouterIcon from 'src/amina_ui/components/RouterIcon.vue'
import PlayerPane from 'src/components/player/PlayerPane.vue'
import FooterToolbar from 'src/components/footer/FooterToolbar.vue'

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
    icon: 'queue_music',
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
    PlayerPane,
    FooterToolbar
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
@font-face
  font-family: Andika
  src: url( '~src/css/fonts/Andika-ZW3x.ttf' )

@font-face
  font-family: Fredoka
  src: url( '~src/css/fonts/Fredoka-Regular.ttf' )

@font-face
  font-family: Parisienne
  src: url( '~src/css/fonts/Parisienne-Regular.ttf' )

.main-layout
  font-family: 'Fredoka', 'Andika'

.header
  background-color: $amina-primary-color

  .root-link
    font-family: 'Parisienne'
    text-shadow: 0px 0px 10px rgba(255,255,255,0.6)
    color: white
    text-decoration: none

.footer
  height: 27px
  background-color: $amina-primary-color

</style>
