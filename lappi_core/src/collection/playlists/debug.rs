use std::fs::File;
use std::path::Path;

use amina_core::service::Context;

use crate::debug::Debugger;

use super::PlaylistsCollection;

pub fn init_playlists_from_csv(context: &Context, playlists: &PlaylistsCollection) {
    let debugger = context.get_service::<Debugger>();

    if debugger.config().collection.init {
        log::debug!("Initializing playlists from csv");

        let folder_path = debugger.get_debug_root_workspace()
            .join(&debugger.config().collection.init_folder);

        log::debug!("Init folder: {:?}", &folder_path);

        init_classic_playlists(context, playlists, &folder_path);
    }
}

fn init_classic_playlists(_context: &Context, playlists: &PlaylistsCollection, init_folder: &Path) {
    let playlists_path = init_folder.join("playlists/classic/playlists.csv");
    let mut playlists_reader = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .from_reader(File::open(playlists_path).unwrap());

    for result in playlists_reader.records() {
        let record = result.unwrap();
        log::trace!("{:?}", record);
        playlists.create_playlist(record.get(0).unwrap().to_string());
    }
}