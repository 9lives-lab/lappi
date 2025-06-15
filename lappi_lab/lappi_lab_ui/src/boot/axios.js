import { boot } from 'quasar/wrappers'
import { AminaClientAPI } from 'src/amina_ui/client_api'

class LappiApi extends AminaClientAPI {
  constructor(apiBaseAddr) {
    super(apiBaseAddr)
  }

  async getInternalFileUrl (internalFileId) {
    const internalPath = await this.sendRequest('lappi.collection.internal_files.get_internal_path', { file_id: internalFileId })
    return this.getFileUrl('lappi.collection.internal/' + internalPath.path )
  }

  async getPictureUrl (pictureId) {
    const pictureDesc = await this.sendRequest('lappi.collection.pictures.get_picture_descriptor', { picture_id: pictureId })
    return this.getInternalFileUrl(pictureDesc.internal_file_id)
  }

}


// Be careful when using SSR for cross-request state pollution
// due to creating a Singleton instance here;
// If any client changes this (global) instance, it might be a
// good idea to move this instance creation inside of the
// "export default () => {}" function below (which runs individually
// for each client)

export default boot(({ app }) => {
  let host = window.location.host
  if (process.env.DEV) {
    host = window.location.hostname + ':8090'
  }

  const lappiApi = new LappiApi(host)
  // for use inside Vue files (Options API) through this.$axios and this.$aminaApi
  app.config.globalProperties.$aminaApi = lappiApi
  app.config.globalProperties.$lappiApi = lappiApi
})
