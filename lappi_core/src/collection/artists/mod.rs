pub mod types;

use std::sync::Arc;

use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::Context;

use crate::collection::artists::types::ArtistDescription;
use crate::collection::database_api::DatabaseApi;
use crate::collection::types::ArtistId;
use crate::database::Database;
use crate::database_api::DbResult;

#[derive(Clone)]
pub struct ArtistsCollection {
    db: Arc<Box<dyn DatabaseApi>>,
}

impl ArtistsCollection {

    pub fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();
        let db_api = Arc::new(database.collection());

        let artists = Arc::new(Self {
            db: db_api,
        });

        register_rpc_handler!(rpc, artists, "lappi.collection.artists.find_by_name", find_by_name(name: String));
        register_rpc_handler!(rpc, artists, "lappi.collection.artists.get_description", get_description(artist_id: ArtistId));

        return artists;
    }

    pub fn batch(db: Arc<Box<dyn DatabaseApi>>) -> Arc<Self> {
        Arc::new(Self {
            db,
        })
    }

    pub fn find_by_name(&self, name: String) -> ArtistId {
        self.db.find_or_add_artist(&name).unwrap()
    }

    pub fn get_description(&self, artist_id: ArtistId) -> DbResult<Box<ArtistDescription>> {
        Ok(Box::new(ArtistDescription {
            name: self.db.get_artist_name(artist_id)?,
        }))
    }

}
