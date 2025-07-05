use anyhow::Result;
use camino::Utf8Path;
use rusqlite::{params, OptionalExtension};

use crate::collection::folders::database_api::FoldersDbApi;
use crate::collection::folders::{FolderDesc, FolderId, FolderType};
use crate::collection::internal_files::InternalFileId;
use crate::collection::music::MusicItemId;
use crate::collection::pictures::PictureId;
use crate::database::sqlite::utils::{DatabaseContext, DatabaseUtils, ProtobufExporter, ProtobufImporter};

pub struct FoldersDb {
    db_utils: DatabaseUtils,
}

impl FoldersDb {
    pub fn new(db_utils: DatabaseUtils) -> Self {
        Self {
            db_utils,
        }
    }
    
    fn i32_to_folder_type(&self, i: i32) -> FolderType {
        match i {
            0 => FolderType::Folder,
            1 => FolderType::Artist,
            2 => FolderType::Album,
            _ => panic!("Unexpected folder type {}", i),
        }
    }

    fn get_folder_description(&self, context: &DatabaseContext, folder_id: FolderId) -> Result<FolderDesc> {
        if folder_id == self.get_root_folder() {
            return Ok(FolderDesc {
                folder_id: 0,
                name: String::from("Root"),
                folder_type: FolderType::Folder,
                avatar_picture_id: None,
            });
        }

        let folder_description = context.connection().query_row(
            "SELECT id, name, folder_type, avatar_picture_id FROM folders WHERE id=(?1)",
            params![folder_id],
            |row| {
                Ok(FolderDesc {
                    folder_id:   row.get::<_, i64>(0)? as FolderId,
                    name:        row.get::<_, String>(1)?,
                    folder_type: self.i32_to_folder_type(row.get::<_, i32>(2)?),
                    avatar_picture_id: row.get::<_, Option<i64>>(3)?,
                })
            },
        )?;

        Ok(folder_description)
    }

    fn find_folder_id(&self, context: &DatabaseContext, parent_id: FolderId, folder_name: &str) -> Result<Option<FolderId>> {
        let result = context.connection().query_row(
            "SELECT id FROM folders WHERE parent_id == (?1) AND name == (?2)",
            params![parent_id, folder_name],
            |row| row.get::<_, i64>(0),
        ).optional()?;
        Ok(result)
    }

    pub fn import(&self, base_path: &Utf8Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut importer = ProtobufImporter::create(base_path, "folders.pb")?;
        let sql = "INSERT INTO folders (id, parent_id, name, folder_type, avatar_picture_id, description_file_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
        while let Some(row) = importer.read_next_row::<crate::proto::collection::FoldersRow>()? {
            db_context.connection().execute(sql, params![
                row.folder_id,
                row.parent_folder_id,
                row.name,
                row.folder_type,
                row.avatar_picture_id,
                row.description_file_id
            ])?;
        }
        Ok(())
    }

    pub fn export(&self, base_path: &Utf8Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut exporter = ProtobufExporter::create(base_path, "folders.pb")?;
        let sql = "SELECT id, parent_id, name, folder_type, avatar_picture_id, description_file_id FROM folders";
        let mut stmt = db_context.connection().prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let mut folders_row = crate::proto::collection::FoldersRow::new();
            folders_row.folder_id = row.get::<_, i64>(0)?;
            folders_row.parent_folder_id = row.get::<_, i64>(1)?;
            folders_row.name = row.get::<_, String>(2)?;
            folders_row.folder_type = row.get::<_, i32>(3)?;
            folders_row.avatar_picture_id = row.get::<_, Option<i64>>(4)?;
            folders_row.description_file_id = row.get::<_, Option<i64>>(5)?;
            Ok(folders_row)
        })?;
        for row in rows {
            exporter.write_row(&row?)?;
        }
        Ok(())
    }
}

impl FoldersDbApi for FoldersDb {
    fn clone_api(&self) -> Box<dyn FoldersDbApi> {
        return Box::new(FoldersDb::new(self.db_utils.clone()));
    }

    fn is_empty(&self) -> bool {
        self.db_utils.lock().is_empty("folders")
    }

    fn get_root_folder(&self) -> FolderId {
        0
    }

    fn get_folder_parent(&self, folder_id: FolderId) -> Result<FolderId> {
        self.db_utils.lock().get_field_value(folder_id, "folders", "parent_id")
    }

    fn get_folder_name(&self, folder_id: FolderId) -> Result<String> {
        self.db_utils.lock().get_field_value(folder_id, "folders", "name")
    }

    fn get_folder_description(&self, folder_id: FolderId) -> Result<FolderDesc> {
        let context = self.db_utils.lock();
        self.get_folder_description(&context, folder_id)
    }

    fn get_description_file(&self, folder_id: FolderId) -> Result<Option<InternalFileId>> {
        self.db_utils.lock().get_field_value(folder_id, "folders", "description_file_id")
    }

    fn set_folder_name(&self, folder_id: FolderId, name: &str) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.set_field_value(folder_id, "folders", "name", name)?;
        context.on_folders_updated(); // Notify any observers of the change
        Ok(())
    }

    fn set_folder_type(&self, folder_id: FolderId, folder_type: FolderType) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.set_field_value(folder_id, "folders", "folder_type", folder_type as i32)?;
        context.on_folders_updated(); // Notify any observers of the change
        Ok(())
    }

    fn set_folder_cover(&self, folder_id: FolderId, picture_id: PictureId) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.set_field_value(folder_id, "folders", "avatar_picture_id", picture_id)?;
        context.on_folders_updated();
        Ok(())
    }

    fn set_description_file(&self, folder_id: FolderId, file_id: InternalFileId) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.set_field_value(folder_id, "folders", "description_file_id", file_id)?;
        context.on_folders_updated();
        Ok(())
    }

    fn find_or_add_folder(&self, parent_id: FolderId, folder_name: &str, folder_type: FolderType) -> Result<FolderId> {
        let mut context = self.db_utils.lock();

        let folder_id = match self.find_folder_id(&context, parent_id, folder_name)? {
            Some(id) => id,
            None => {
                context.connection().execute(
                    "INSERT INTO folders (parent_id, name, folder_type) VALUES (?1, ?2, ?3)",
                    params![parent_id, folder_name, folder_type as i32],
                )?;
                context.on_folders_updated();
                context.connection().last_insert_rowid()
            }
        };

        return Ok(folder_id);
    }

    fn get_folders_in_folder(&self, folder_id: FolderId) -> Result<Vec<FolderDesc>> {
        let context = self.db_utils.lock();
        let id_list = context.get_fields_list_by_field_i64_value("folders", "id", "parent_id", folder_id)?;
        let mut result = Vec::new();
        for folder_id in id_list {
            result.push(self.get_folder_description(&context, folder_id)?);
        }
        Ok(result)
    }

    fn get_music_items_in_folder(&self, folder_id: FolderId) -> Result<Vec<MusicItemId>> {
        self.db_utils.lock().get_fields_list_by_field_i64_value("music_items", "id", "folder_id", folder_id)
    }
}
