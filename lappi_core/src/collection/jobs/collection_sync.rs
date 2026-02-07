use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};
use std::sync::Arc;

use amina_core::service::Service;
use amina_core::service::Context as AppContext;
use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};

use crate::database::sqlite::utils::ProtobufImporter;
use crate::proto::collection::InternalFilesRow;
use crate::storage::remote::RemoteStorage;
use crate::storage::remote::adapter::{DummyRemoteStorage, RemoteStorageAdapter};
use crate::workspace::Workspace;
use crate::jobs::{JobContext, JobDescription, JobFactory, Jobs};
use crate::collection::Collection;
use crate::collection::internal_files::InternalFileId;

#[derive(PartialEq, Eq)]
enum SyncMode {
    Download,
    Upload,
}

struct CollectionSyncJob {
    job_ctx: Arc<JobContext>,
    collection: Service<Collection>,
    remote_storage: Box<dyn RemoteStorageAdapter>,
    sync_mode: SyncMode,
    files_to_copy: Vec<Utf8PathBuf>,
    files_for_remove: Vec<Utf8PathBuf>,
}

impl CollectionSyncJob {
    fn create(job_ctx: Arc<JobContext>, sync_mode: SyncMode) -> Result<Self> {
        let collection = crate::context().get_service::<Collection>();
        let remote_storage = Box::new(DummyRemoteStorage::new());

        Ok(Self {
            job_ctx,
            collection,
            remote_storage,
            sync_mode,
            files_to_copy: Vec::new(),
            files_for_remove: Vec::new(),
        })
    }

    fn get_temp_path() -> Utf8PathBuf {
        let workspace = crate::context().get_service::<Workspace>();
        let mut path = workspace.get_temp_dir();
        path.push("sync_job");
        return path;
    }

    fn ensure_directory_exists(&self, path: &Utf8PathBuf) -> Result<()> {
        let parent_path = path.parent().ok_or_else(|| anyhow::anyhow!("Failed to get parent directory of {:?}", path))?;
        if !parent_path.exists() {
            std::fs::create_dir_all(parent_path).context(format!("Failed to create directory {:?}", parent_path))?;
        }
        Ok(())
    }

    fn get_local_storage_path(&self, internal_path: &Utf8Path) -> Utf8PathBuf {
        self.collection.get_local_path().join(internal_path)
    }

    fn get_remote_storage_temp_path(&self, internal_path: &Utf8Path) -> Utf8PathBuf {
        let mut path = Self::get_temp_path();
        path.push("remote_files");
        path.push(internal_path);
        return path;
    }

    fn set_progress(&self, progress: f32, title: &str) {
        self.job_ctx.set_progress(progress, title.to_string());
    }

    fn add_file_for_copy(&mut self, file_path: &Utf8Path) -> Result<()> {
        log::debug!("Add file to copy '{}'", file_path);
        self.files_to_copy.push(file_path.to_owned());
        Ok(())
    }

    fn add_file_for_remove(&mut self, file_path: &Utf8Path) -> Result<()> {
        log::debug!("Add file for remove. file path {}", file_path);
        self.files_for_remove.push(file_path.to_owned());
        Ok(())
    }

    fn download_file_to_temp_folder(&mut self, internal_path: &Utf8Path) -> Result<()> {
        log::debug!("Read remote file to temp folder '{}'", internal_path);
        let temp_file_path = self.get_remote_storage_temp_path(internal_path);
        self.ensure_directory_exists(&temp_file_path)?;
        let mut temp_file = File::create(&temp_file_path)?;
        self.remote_storage.read_file(internal_path, &mut temp_file)?;
        Ok(())
    }

    fn format_temp_folder(&self) -> Result<()> {
        let temp_path = Self::get_temp_path();
        log::info!("Formatting the temporary folder {}", temp_path);
        if temp_path.exists() {
            std::fs::remove_dir_all(&temp_path)?;
        }
        std::fs::create_dir_all(&temp_path)?;
        Ok(())
    }

    fn update_hashes(&self) -> Result<()> {
        log::info!("Update file hashes");
        self.set_progress(0.0, "Update file hashes");
        self.collection.internal_files().update_file_hashes()?;
        Ok(())
    }

    fn save_collection(&self) -> Result<()> {
        log::info!("Save collection");
        self.set_progress(0.0, "Save collection");
        self.collection.save();
        Ok(())
    }

    fn open_remote_connection(&mut self) -> Result<()> {
        log::info!("Connect to remote server");
        self.set_progress(0.0, "Connect to remote server");
        let remote_storage = crate::context().get_service::<RemoteStorage>();
        self.remote_storage = remote_storage.connect()?;
        Ok(())
    }

    fn read_hash_file(&self, file_path: &Utf8Path) -> Result<Vec<u8>> {
        let mut hash = Vec::new();
        let mut file = File::open(&file_path)?;
        file.read_to_end(&mut hash)?;
        Ok(hash)
    }

    fn process_meta_file(&mut self, file_name: &str) -> Result<()> {
        let meta_file_path = Utf8PathBuf::from(".lappi/meta").join(file_name);
        let meta_hash_path = Utf8PathBuf::from(".lappi/meta").join(format!("{}.hash", file_name));

        match self.download_file_to_temp_folder(&meta_hash_path) {
            Ok(_) => {
                let remote_meta_hash_path = self.get_remote_storage_temp_path(&meta_hash_path);
                let remote_hash = self.read_hash_file(&remote_meta_hash_path)?;

                let local_meta_hash_path = self.get_local_storage_path(&meta_hash_path);
                match self.read_hash_file(&local_meta_hash_path) {
                    Ok(local_hash) => {
                        if local_hash == remote_hash {
                            log::debug!("Meta file is up to date '{}'", &meta_file_path);
                        }
                        else
                        {
                            self.add_file_for_copy(&meta_file_path)?;
                            self.add_file_for_copy(&meta_hash_path)?;
                        }
                    },
                    Err(err) => {
                        match self.sync_mode {
                            SyncMode::Download => {
                                self.add_file_for_copy(&meta_file_path)?;
                                self.add_file_for_copy(&meta_hash_path)?;
                            },
                            SyncMode::Upload => {
                                return Err(err);
                            },
                        }
                    },
                }
            },
            Err(err) => {
                log::info!("Failed to download {}", &meta_hash_path);
                match self.sync_mode {
                    SyncMode::Download => {
                        return Err(err);
                    },
                    SyncMode::Upload => {
                        self.add_file_for_copy(&meta_file_path)?;
                        self.add_file_for_copy(&meta_hash_path)?;
                    },
                }
            },
        }

        Ok(())
    }

    fn process_meta_files(&mut self) -> Result<()> {
        let meta_files_list = vec![
            "internal_files.pb",
            "folders.pb",
            "music_items.pb",
            "lyrics.pb",
            "picture_items.pb",
            "tags.pb",
            "music_files.pb",
            "music_links.pb",
            "playlists.pb",
            "playlist_items.pb"
        ];

        for (i, meta_file_name) in meta_files_list.iter().enumerate() {
            self.set_progress(0.0, &format!("Process meta files [{}/{}]", i + 1, meta_files_list.len()));
            self.process_meta_file(meta_file_name)?;
        }

        Ok(())
    }

    fn create_internal_files_map(&self, db_file_path: &Utf8Path) -> Result<HashMap<InternalFileId, InternalFilesRow>> {
        let mut internal_files_map = HashMap::new();
        let mut importer = ProtobufImporter::create(db_file_path)?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::InternalFilesRow>()? {
            internal_files_map.insert(row.file_id, row);
        }
        Ok(internal_files_map)
    }

    fn create_remote_internal_files_map(&mut self) -> Result<HashMap<InternalFileId, InternalFilesRow>> {
        let internal_files_db_path = Utf8PathBuf::from(".lappi/meta/internal_files.pb");
        if let Err(_) = self.download_file_to_temp_folder(&internal_files_db_path) {
            return Ok(HashMap::new());
        }
        let db_file_path = self.get_remote_storage_temp_path(&internal_files_db_path);
        self.create_internal_files_map(&db_file_path)
    }

    fn create_local_internal_files_map(&mut self) -> Result<HashMap<InternalFileId, InternalFilesRow>> {
        let internal_files_db_path = Utf8PathBuf::from(".lappi/meta/internal_files.pb");
        let db_file_path = self.get_local_storage_path(&internal_files_db_path);
        self.create_internal_files_map(&db_file_path)
    }

    fn process_internal_files(&mut self) -> Result<()> {
        self.set_progress(0.0, "Process internal files");

        let remote_files_map = self.create_remote_internal_files_map()?;
        let local_files_map = self.create_local_internal_files_map()?;

        let (src_files_map, dst_files_map) = match self.sync_mode {
            SyncMode::Download => (remote_files_map, local_files_map),
            SyncMode::Upload => (local_files_map, remote_files_map),
        };

        for (src_file_id, src_file_desc) in &src_files_map {
            let src_path = Utf8PathBuf::from(&src_file_desc.internal_path);
            match dst_files_map.get(src_file_id) {
                Some(dst_file_desc) => {
                    if src_file_desc.internal_path == dst_file_desc.internal_path {
                        if src_file_desc.hash == src_file_desc.hash {
                            log::debug!("Internal file is up to date '{}", &src_path);
                        } else {
                            // File changed
                            self.add_file_for_copy(&src_path)?;
                        }
                    } else {
                        // File moved
                        let dst_path = Utf8PathBuf::from(&dst_file_desc.internal_path);
                        self.add_file_for_remove(&dst_path)?;
                        self.add_file_for_copy(&src_path)?;
                    }
                },
                None => {
                    // File does not exist
                    self.add_file_for_copy(&src_path)?;
                },
            }
        }

        for (dst_file_id, dst_file_desc) in &dst_files_map {
            let dst_path = Utf8PathBuf::from(&dst_file_desc.internal_path);
            match src_files_map.get(dst_file_id) {
                Some(_) => {

                },
                None => {
                    // File does not exist
                    self.add_file_for_remove(&dst_path)?;
                },
            }
        }
        
        Ok(())
    }

    fn copy_files(&mut self) -> Result<()> {
        let action = match self.sync_mode {
            SyncMode::Download => "Downloading",
            SyncMode::Upload => "Uploading",
        };
        let files_num = self.files_to_copy.len();
        for (index, file_path) in self.files_to_copy.iter().enumerate() {
            let progress = index as f32 / files_num as f32;
            let title = format!("{} files {}/{}", action, index, files_num);
            self.set_progress(progress, &title);

            let local_path = self.get_local_storage_path(file_path);
            log::info!("{} file {}", action, &file_path);

            match self.sync_mode {
                SyncMode::Download => {
                    self.ensure_directory_exists(&local_path)?;
                    let mut local_file = File::create(&local_path)?;
                    self.remote_storage.read_file(file_path, &mut local_file)?;
                },
                SyncMode::Upload => {
                    let mut local_file = File::open(&local_path)?;
                    self.remote_storage.write_file(file_path, &mut local_file)?;
                },
            };
        }

        Ok(())
    }

    fn remove_files(&mut self) -> Result<()> {
        let files_num = self.files_for_remove.len();
        for (index, file_path) in self.files_for_remove.iter().enumerate() {
            let progress = index as f32 / files_num as f32;
            let title = format!("Removing files {}/{}", index, files_num);
            self.set_progress(progress, &title);
            log::info!("Removing file {}", &file_path);

            match self.sync_mode {
                SyncMode::Download => {
                    let local_path = self.get_local_storage_path(file_path);
                    std::fs::remove_file(&local_path).context(format!("Failed to remove file {:?}", local_path))?;
                },
                SyncMode::Upload => {
                    self.remote_storage.remove_file(file_path)?;
                },
            };
        }

        Ok(())
    }

    fn reload_collection(&mut self) -> Result<()> {
        if self.sync_mode != SyncMode::Download {
            return Ok(())
        }


        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        self.format_temp_folder()?;
        self.update_hashes()?;
        self.save_collection()?;
        self.open_remote_connection()?;
        self.process_meta_files()?;
        self.process_internal_files()?;
        self.copy_files()?;
        self.remove_files()?;
        self.reload_collection()?;
        //self.format_temp_folder()?;
        self.set_progress(1.0, "Done");

        Ok(())
    }
}

struct CollectionUploadJobFactory {

}

impl CollectionUploadJobFactory {
    fn create(_: &AppContext) -> Box<Self> {
        Box::new(Self {

        })
    }
}

impl JobFactory for CollectionUploadJobFactory {
    fn get_description(&self) -> Box<JobDescription> {
        let name = "Collection upload";

        Box::new(JobDescription {
            job_id: name,
            name,
            icon: "drive_file_move",
            description: "Upload collection to remote server.",
        })
    }

    fn is_always_ready(&self) -> bool {
        true
    }

    fn run(&self, job_ctx: Arc<JobContext>) -> Result<()> {
        let mut job = CollectionSyncJob::create(job_ctx, SyncMode::Upload)?;
        job.run()
    }
}

struct CollectionDownloadJobFactory {

}

impl CollectionDownloadJobFactory {
    fn create(_: &AppContext) -> Box<Self> {
        Box::new(Self {

        })
    }
}

impl JobFactory for CollectionDownloadJobFactory {
    fn get_description(&self) -> Box<JobDescription> {
        let name = "Collection download";

        Box::new(JobDescription {
            job_id: name,
            name,
            icon: "drive_file_move",
            description: "Download collection from remote server.",
        })
    }

    fn is_always_ready(&self) -> bool {
        true
    }

    fn run(&self, job_ctx: Arc<JobContext>) -> Result<()> {
        let mut job = CollectionSyncJob::create(job_ctx, SyncMode::Download)?;
        job.run()
    }
}

pub fn initialize() {
    let context = crate::context();
    let jobs = context.get_service::<Jobs>();
    jobs.register_job(CollectionUploadJobFactory::create(context));
    jobs.register_job(CollectionDownloadJobFactory::create(context));
}

