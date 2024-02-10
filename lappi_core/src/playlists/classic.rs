use std::sync::Arc;

use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, ServiceApi, ServiceInitializer};

use crate::database::Database;
use crate::playlists::database_api::DatabaseApi;
use crate::playlists::types::{PlaylistDesc, PlaylistId};

pub struct ClassicPlaylists {
    db: Box<dyn DatabaseApi>,
}

impl ClassicPlaylists {

    pub fn get_playlists(&self) -> Vec<PlaylistDesc> {
        return self.db.get_classic_playlists().unwrap();
    }

    pub fn create_playlist(&self, name: String) -> PlaylistId {
        return self.db.create_classic_playlist(&name).unwrap();
    }

}

impl ServiceApi for ClassicPlaylists {

}

impl ServiceInitializer for ClassicPlaylists {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();

        let playlists = Arc::new(Self {
            db: database.playlists(),
        });

        register_rpc_handler!(rpc, playlists, "lappi.playlists.classic.get_playlist", get_playlists());
        register_rpc_handler!(rpc, playlists, "lappi.playlists.classic.create_playlist", create_playlist(name: String));

        return playlists;
    }
}
