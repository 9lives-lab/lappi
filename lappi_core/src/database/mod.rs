pub mod sqlite;

use std::ops::Deref;
use std::sync::Arc;

use amina_core::service::{ServiceApi, ServiceInitializer, Context};

use crate::collection::database_api::CollectionDbApi;

pub struct Database {
    api: Box<dyn CollectionDbApi>,
}

impl Deref for Database {
    type Target = dyn CollectionDbApi;

    fn deref(&self) -> &Self::Target {
        self.api.deref()
    }
}

impl ServiceApi for Database {

}

impl ServiceInitializer for Database {
    fn initialize(context: &Context) -> Arc<Self> {
        let db_api = sqlite::initialize(context);

        Arc::new(Database {
            api: Box::new(db_api),
        })
    }
}
