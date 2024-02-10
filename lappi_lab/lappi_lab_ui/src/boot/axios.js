import { boot } from 'quasar/wrappers'
import aminaApi from 'src/amina_ui/client_api.js'

// Be careful when using SSR for cross-request state pollution
// due to creating a Singleton instance here;
// If any client changes this (global) instance, it might be a
// good idea to move this instance creation inside of the
// "export default () => {}" function below (which runs individually
// for each client)

export default boot(({ app }) => {
  // for use inside Vue files (Options API) through this.$axios and this.$aminaApi
  app.config.globalProperties.$aminaApi = aminaApi
})
