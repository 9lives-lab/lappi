use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::Arc;

use amina_core::service::{Context, Service};
use anyhow::Result;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

use crate::workspace::Workspace;
use crate::jobs::{JobContext, JobDescription, JobFactory, Jobs};
use crate::collection::Collection;
use crate::collection::internal_files::{InternalFileId, InternalPath};


fn get_description() -> Box<JobDescription> {
    let name = "Collection migration";
    let icon = "drive_file_move";
    let description = "Go through all items in the collection and move files according to the storage scheme.";

    Box::new(JobDescription {
        job_id: name,
        name,
        icon,
        description,
    })
}

#[derive(Serialize, Deserialize)]
struct MigrationEntry {
    file_id: InternalFileId,
    new_path: String,
}

struct CollectionMigrationJob {
    job_ctx: Arc<JobContext>,
    collection: Service<Collection>,
    migration_list_file: File,
    migration_list_size: i32,
}

impl CollectionMigrationJob {
    fn get_migration_list_path() -> Utf8PathBuf {
        let workspace = crate::context().get_service::<Workspace>();
        let mut migration_list_file_path = workspace.get_temp_dir();
        migration_list_file_path.push("migration_list.txt");
        return migration_list_file_path;
    }

    fn create(job_ctx: Arc<JobContext>) -> Result<Self> {
        let collection = crate::context().get_service::<Collection>();

        let migration_list_path = Self::get_migration_list_path();
        let migration_list_file = File::create(migration_list_path)?;

        Ok(Self {
            job_ctx,
            collection,
            migration_list_file,
            migration_list_size: 0,
        })
    }

    fn set_progress(&mut self, progress: f32, title: &str) {
        self.job_ctx.set_progress(progress, title.to_string());
    }

    fn add_entry(&mut self, file_id: InternalFileId, new_path: InternalPath) -> Result<()> {
        log::debug!("Add migration entry. file id {}, new_path {}", file_id, new_path.as_str());

        let line = serde_json::to_string(&MigrationEntry {
            file_id, new_path: new_path.into()
        })?;

        write!(self.migration_list_file, "{}\n", line)?;
        self.migration_list_size += 1;

        Ok(())
    }

    fn process_folders_description(&mut self) -> Result<()> {
        self.set_progress(0.0, "Process folders description");
        let folders = self.collection.folders().get_all_folders()?;
        for folder_id in folders {
            let file_id = self.collection.folders().get_description_file(folder_id)?;
            if let Some(file_id) = file_id {
                let current_path = self.collection.internal_files().get_internal_path(file_id)?;
                let new_path = self.collection.folders().gen_description_internal_path(folder_id)?;
                if current_path != new_path {
                    self.add_entry(file_id, new_path)?;
                }
            }
        }
        Ok(())
    }

    fn process_music_files(&mut self) -> Result<()> {
        self.set_progress(0.0, "Process music files");
        let music_items = self.collection.music().get_all_music_items()?;
        for music_item_id in music_items {
            let current_description = self.collection.music_sources().get_music_file(music_item_id)?;
            if let Some(description) = current_description {
                let file_id = description.internal_file_id;
                let current_path = self.collection.internal_files().get_internal_path(file_id)?;
                let new_path = self.collection.music_sources().gen_internal_path(music_item_id)?;
                if let Some(new_path) = new_path {
                    if current_path != new_path {
                        self.add_entry(file_id, new_path)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn process_lyrics_files(&mut self) -> Result<()> {
        self.set_progress(0.0, "Process lyrics files");
        let lyrics_list = self.collection.lyrics().get_all_lyrics_list()?;
        for lyrics_id in lyrics_list {
            let descriptor = self.collection.lyrics().get_lyrics_descriptor(lyrics_id)?;
            let file_id = descriptor.internal_file_id;
            let current_path = self.collection.internal_files().get_internal_path(file_id)?;
            let new_path = self.collection.lyrics().gen_internal_path(lyrics_id)?;
            if current_path != new_path {
                self.add_entry(file_id, new_path)?;
            }
        }
        Ok(())
    }

    fn process_pictures_files(&mut self) -> Result<()> {
        self.set_progress(0.0, "Process pictures files");
        let pictures = self.collection.pictures().get_all_pictures()?;
        for picture_id in pictures {
            let descriptor = self.collection.pictures().get_picture_descriptor(picture_id)?;
            let file_id = descriptor.internal_file_id;
            let current_path = self.collection.internal_files().get_internal_path(file_id)?;
            let new_path = self.collection.pictures().gen_internal_path(picture_id)?;
            if current_path != new_path {
                self.add_entry(file_id, new_path)?;
            }
        }
        Ok(())
    }

    fn prepare_entries_list(&mut self) -> Result<()> {
        self.process_folders_description()?;
        self.process_music_files()?;
        self.process_lyrics_files()?;
        self.process_pictures_files()?;
        Ok(())
    }

    fn move_files(&mut self) -> Result<()> {
        self.migration_list_file.flush()?;

        let migration_list_path = Self::get_migration_list_path();
        let migration_list_file = File::open(migration_list_path)?;

        let mut reader = BufReader::new(migration_list_file);
        let mut buf = String::new();

        for i in 0..self.migration_list_size {
            if self.job_ctx.is_interrupted() {
                return Ok(())
            }

            let progress = (i as f32) / (self.migration_list_size as f32);
            let title = format!("Move files {}/{}", i, self.migration_list_size);
            self.set_progress(progress, title.as_str());

            reader.read_line(&mut buf)?;
            let entry: MigrationEntry = serde_json::from_str(&buf)?;
            buf.clear();

            let new_path = InternalPath::from_string(entry.new_path);
            self.collection.internal_files().move_file(entry.file_id, &new_path)?;

        }

        Ok(())
    }

    fn remove_empty_folders(&mut self) -> Result<()> {
        self.set_progress(1.0, "Clear empty folders");
        self.collection.internal_files().remove_empty_folders()
    }

    fn run(&mut self) -> Result<()> {
        self.prepare_entries_list()?;
        self.move_files()?;
        self.remove_empty_folders()?;
        self.set_progress(1.0, "Done");
        Ok(())
    }
}

struct CollectionMigrationJobFactory {

}

impl CollectionMigrationJobFactory {
    fn create(_: &Context) -> Box<Self> {
        Box::new(Self {

        })
    }
}

impl JobFactory for CollectionMigrationJobFactory {
    fn get_description(&self) -> Box<JobDescription> {
        get_description()
    }

    fn is_always_ready(&self) -> bool {
        true
    }

    fn run(&self, job_ctx: Arc<JobContext>) -> Result<()> {
        let mut job = CollectionMigrationJob::create(job_ctx)?;
        job.run()
    }
}

pub fn initialize() {
    let context = crate::context();
    let jobs = context.get_service::<Jobs>();
    jobs.register_job(CollectionMigrationJobFactory::create(context));
}

