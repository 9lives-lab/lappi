import { boot } from 'quasar/wrappers'
import { AminaClientAPI } from 'src/amina_ui/client_api'

// Be careful when using SSR for cross-request state pollution
// due to creating a Singleton instance here;
// If any client changes this (global) instance, it might be a
// good idea to move this instance creation inside of the
// "export default () => {}" function below (which runs individually
// for each client)

export default boot(({ app }) => {
  let host = window.location.host
  if (process.env.DEV) {
    host = 'localhost:8090'
  }

  const aminaApi = new AminaClientAPI(host)
  // for use inside Vue files (Options API) through this.$axios and this.$aminaApi
  app.config.globalProperties.$aminaApi = aminaApi
})
