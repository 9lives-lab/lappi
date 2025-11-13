const routes = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      { path: '', component: () => import('pages/IndexPage.vue') },
      { path: 'collection_tree', component: () => import('pages/CollectionPage.vue') },
      { path: 'playlists', component: () => import('pages/PlaylistsPage.vue') },
      { path: 'file_manager', component: () => import('pages/FileManagerPage.vue') },
      { path: 'jobs', component: () => import('pages/JobsPage.vue') },
      { path: 'settings', component: () => import('pages/SettingsPage.vue') },
      { path: 'commands', component: () => import('pages/CommandsPage.vue') }
    ]
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue')
  }
]

export default routes
