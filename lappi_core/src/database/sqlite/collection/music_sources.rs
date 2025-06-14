use std::path::Path;

use anyhow::Result;
use num_traits::FromPrimitive;
use rusqlite::params;
use protobuf::EnumOrUnknown;
use protobuf::Enum;

use crate::collection::music::{MusicItemId};
use crate::collection::music_sources::database_api::MusicSourcesDbApi;
use crate::collection::music_sources::MusicFileDesc;
use crate::collection::music_sources::MusicFileType;
use crate::collection::music_sources::{MusicLinkDesc, MusicLinkId, MusicLinkType};
use crate::database::sqlite::utils::{DatabaseUtils, ProtobufExporter, ProtobufImporter};

pub struct MusicSourcesDb {
    db_utils: DatabaseUtils,
}

impl MusicSourcesDb {
    pub fn new(db_utils: DatabaseUtils) -> Self {
        Self {
            db_utils,
        }
    }

    pub fn import(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();

        let mut importer = ProtobufImporter::create(base_path, "music_files.pb")?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::MusicFilesRow>()? {
            db_context.connection().execute(
                "INSERT INTO music_files (id, internal_file_id, file_type) VALUES (?1, ?2, ?3)",
                params![row.id, row.internal_file_id, row.file_type.value()],
            )?;
        }

        let mut importer = ProtobufImporter::create(base_path, "music_links.pb")?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::MusicLinksRow>()? {
            db_context.connection().execute(
                "INSERT INTO music_links (id, music_item_id, link, link_type) VALUES (?1, ?2, ?3, ?4)",
                params![row.id, row.music_item_id, row.link, row.link_type.value()],
            )?;
        }

        Ok(())
    }

    pub fn export(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();

        let mut exporter = ProtobufExporter::create(base_path, "music_files.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, internal_file_id, file_type FROM music_files")?;
        let rows = stmt.query_map([], |row| {
            let mut music_items_row = crate::proto::collection::MusicFilesRow::new();
            music_items_row.id = row.get::<_, i64>(0)?;
            music_items_row.internal_file_id = row.get::<_, i64>(1)?;
            music_items_row.file_type = EnumOrUnknown::new(crate::proto::collection::MusicFileType::from_i32(row.get::<_, i32>(2)?).unwrap());
            Ok(music_items_row)
        })?;
        for row in rows {
            exporter.write_row(&row?)?;
        }

        let mut exporter = ProtobufExporter::create(base_path, "music_links.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, music_item_id, link, link_type FROM music_links")?;
        let rows = stmt.query_map([], |row| {
            let mut music_src_links_row = crate::proto::collection::MusicLinksRow::new();
            music_src_links_row.id = row.get::<_, i64>(0)?;
            music_src_links_row.music_item_id = row.get::<_, i64>(1)?;
            music_src_links_row.link = row.get::<_, String>(2)?;
            music_src_links_row.link_type = EnumOrUnknown::new(crate::proto::collection::MusicLinkType::from_i32(row.get::<_, i32>(3)?).unwrap());
            Ok(music_src_links_row)
        })?;
        for row in rows {
            exporter.write_row(&row?)?;
        }

        Ok(())
    }
}

impl MusicSourcesDbApi for MusicSourcesDb {
    fn clone_api(&self) -> Box<dyn MusicSourcesDbApi> {
        return Box::new(MusicSourcesDb::new(self.db_utils.clone()));
    }

    fn add_music_file(&self, descriptor: &MusicFileDesc) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.connection().execute(
            "INSERT INTO music_files (id, internal_file_id, file_type) VALUES (?1, ?2, ?3)",
            params![descriptor.music_item_id, descriptor.internal_file_id, descriptor.file_type as i32],
        )?;
        context.on_folders_updated(); 
        Ok(())
    }

    fn get_music_file(&self, item_id: MusicItemId) -> Result<Option<MusicFileDesc>> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare(
            "SELECT id, internal_file_id, file_type FROM music_files WHERE id=(?1)"
        )?;
        let rows = stmt.query_map(
            params![item_id], |row| Ok(
                MusicFileDesc {
                    music_item_id: row.get::<_, i64>(0)? as MusicItemId,
                    internal_file_id: row.get::<_, i64>(1)?,
                    file_type: MusicFileType::from_i32(row.get::<_, i32>(2)?).unwrap(),
                }
            )
        )?;
        if let Some(result) = rows.last() {
            return Ok(Some(result?))
        } else {
            return Ok(None);
        }
    }

    fn delete_music_file(&self, item_id: MusicItemId) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.remove_row("music_files", item_id)?;
        context.on_music_updated();
        Ok(())
    }


    fn add_music_link(&self, item_id: MusicItemId, link_type: MusicLinkType, link: &str) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.connection().execute(
            "INSERT INTO music_links (music_item_id, link, link_type) VALUES (?1, ?2, ?3)",
            params![item_id, link, link_type as i32],
        )?;
        context.on_music_updated();
        Ok(())
    }

    fn set_music_link(&self, link_id: MusicLinkId, link: &str) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.set_field_value(link_id, "music_links", "link", link)?;
        context.on_music_updated();
        Ok(())
    }

    fn get_music_links(&self, item_id: MusicItemId) -> Result<Vec<MusicLinkDesc>> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare(
            "SELECT id, music_item_id, link, link_type FROM music_links WHERE music_item_id=(?1)"
        )?;
        let rows = stmt.query_map(
            params![item_id], |row| Ok(
                MusicLinkDesc {
                    id: row.get::<_, i32>(0)? as MusicLinkId,
                    music_item_id: row.get::<_, i32>(1)? as MusicItemId,
                    link: row.get::<_, String>(2)?,
                    link_type: MusicLinkType::from_i32(row.get::<_, i32>(3)?).unwrap(),
                }
            )
        )?;
        let mut result = Vec::new();
        for row in rows {
            log::debug!("Music source: {:?}", &row);
            result.push(row?);
        }
        Ok(result)
    }

    fn delete_music_link(&self, link_id: MusicLinkId) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.remove_row("music_links", link_id)?;
        context.on_music_updated();
        Ok(())
    }
}
