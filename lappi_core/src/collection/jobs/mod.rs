pub mod collection_migration;
pub mod collection_sync;

pub fn initialize() {
    collection_migration::initialize();
    collection_sync::initialize();
}

