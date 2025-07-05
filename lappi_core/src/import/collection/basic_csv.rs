use std::fs::File;

use amina_core::service::Service;
use anyhow::Result;
use camino::Utf8Path;

use crate::collection::Collection;
use crate::collection::tags::TagsMap;

pub struct BasicCsvCollectionImporter {
    collection: Service<Collection>,
}

impl BasicCsvCollectionImporter {

    pub fn new() -> Self {
        Self {
            collection: crate::context().get_service::<Collection>(),
        }
    }

    fn import_items(&self, dir_path: &Utf8Path) -> Result<()> {
        let mut file_path = dir_path.to_path_buf();
        file_path.push("items.csv");
 
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'|')
            .from_reader(File::open(file_path)?);

        let collection = self.collection.clone();
        collection.start_batch();
        for result in reader.records() {
            let record = result?;
            log::trace!("{:?}", record);
            let mut tags = TagsMap::new();
            tags.add_string_tag("artist", record.get(0).unwrap().to_string());
            tags.add_string_tag("album", record.get(1).unwrap().to_string());
            tags.add_string_tag("title", record.get(4).unwrap().to_string());
            crate::import::collection::utils::import_song(&collection, &tags)?;
        }
        collection.stop_batch();
        Ok(())
    }

    pub fn import(&self, path: &Utf8Path) -> Result<()> {
        self.import_items(path)
    }

}

