pub mod sqlite;

use std::sync::Arc;
use amina_core::service::{ServiceApi, ServiceInitializer, Context};

pub struct DatabaseApiList {
    pub collection_api: Box<dyn crate::collection::database_api::DatabaseApi>,
    pub playlists_api: Box<dyn crate::playlists::database_api::DatabaseApi>,
}

pub struct Database {
    api_list: DatabaseApiList,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            api_list: DatabaseApiList {
                collection_api: self.api_list.collection_api.clone_api(),
                playlists_api: self.api_list.playlists_api.clone_api(),
            }
        }
    }
}

impl Database {

    pub fn collection(&self) -> Box<dyn crate::collection::database_api::DatabaseApi> {
        self.api_list.collection_api.clone_api()
    }

    pub fn playlists(&self) -> Box<dyn crate::playlists::database_api::DatabaseApi> {
        self.api_list.playlists_api.clone_api()
    }

}

impl ServiceApi for Database {

}

impl ServiceInitializer for Database {

    fn initialize(context: &Context) -> Arc<Self> {
        let db_api = sqlite::initialize(context);

        Arc::new(Database {
            api_list: db_api,
        })
    }

}
