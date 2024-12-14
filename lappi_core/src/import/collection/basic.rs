use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use crate::collection::Collection;
use crate::collection::tags::TagsMap;

pub struct BasicCollectionImporter {
    collection: Arc<Collection>,
}

impl BasicCollectionImporter {

    pub fn new(collection: Arc<Collection>) -> Self {
        Self {
            collection
        }
    }

    fn import_items(&self, dir_path: &Path) {
        let mut file_path = dir_path.to_path_buf();
        file_path.push("items.csv");
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'|')
            .from_reader(File::open(file_path).unwrap());

        let collection = self.collection.clone();
        collection.start_batch();
        for result in reader.records() {
            let record = result.unwrap();
            log::trace!("{:?}", record);
            let mut tags = TagsMap::new();
            tags.add_string_tag("artist", record.get(0).unwrap().to_string());
            tags.add_string_tag("album", record.get(1).unwrap().to_string());
            tags.add_string_tag("title", record.get(4).unwrap().to_string());
            crate::import::collection::utils::import_song(&collection, &tags);
        }
        collection.stop_batch();
    }

    pub fn import(&self, path: &Path) {
        self.import_items(path);
    }

}

