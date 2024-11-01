pub mod database_api;
pub mod types;
pub mod debug;

use std::sync::Arc;

use amina_core::service::Context;
use amina_core::rpc::Rpc;
use amina_core::register_rpc_handler;

use database_api::PlaylistsDbApi;
use types::{PlaylistDesc, PlaylistId};

use crate::database::Database;

pub struct PlaylistsCollection {
    db: Box<dyn PlaylistsDbApi>,
}

impl PlaylistsCollection {
    pub fn get_playlists(&self) -> Vec<PlaylistDesc> {
        return self.db.get_classic_playlists().unwrap();
    }

    pub fn create_playlist(&self, name: String) -> PlaylistId {
        return self.db.create_classic_playlist(&name).unwrap();
    }
}

impl PlaylistsCollection {
    pub fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();

        let playlists = Arc::new(Self {
            db: database.get_playlist(),
        });

        register_rpc_handler!(rpc, playlists, "lappi.playlists.classic.get_playlist", get_playlists());
        register_rpc_handler!(rpc, playlists, "lappi.playlists.classic.create_playlist", create_playlist(name: String));

        return playlists;
    }
}
