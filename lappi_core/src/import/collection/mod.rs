pub mod basic_csv;
pub mod basic_yaml;
pub mod utils;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use walkdir::WalkDir;
use amina_core::cmd_manager::{ArgDescription, ArgType, CmdDescription, CmdManager};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};
use amina_core::tasks::{TaskContext, TaskManager};

use crate::collection::Collection;
use crate::collection::music::SourceType;
use crate::collection::tags::TagsMap;
use crate::platform_api::PlatformApi;
use crate::metadata;

trait ImportLogger {
    fn log_song(&mut self, tags: &TagsMap);
}

struct DummyLogger {

}

impl DummyLogger {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl ImportLogger for DummyLogger {
    fn log_song(&mut self, _tags: &TagsMap) {

    }
}

struct CsvLogger {
    file: File,
}

impl CsvLogger {

    pub fn new(path: &Path) -> Self {
        Self {
            file: File::options().append(true).create(true).open(path).unwrap()
        }
    }

    fn tag_to_string(tags: &TagsMap, key: &str) -> String {
        let tag_option = tags.get_string_tag(key);
        match tag_option {
            Some(text) => text.clone(),
            None => "".to_string()
        }
    }

}

impl ImportLogger for CsvLogger {
    fn log_song(&mut self, tags: &TagsMap) {
        let artist = Self::tag_to_string(tags, "artist");
        let album = Self::tag_to_string(tags, "album");
        let title = Self::tag_to_string(tags, "title");
        let line = format!("{artist}|{album}|{title}|\n");
        self.file.write_all(line.as_bytes()).unwrap();
    }
}

trait Importer: Send + Sync {
    fn import(&self, path: &Path, logger: &mut dyn ImportLogger);
}

#[derive(Clone)]
struct AudioImporter {
    collection: Service<Collection>,
}

impl Importer for AudioImporter {
    fn import(&self, path: &Path, logger: &mut dyn ImportLogger) {
        let metadata = metadata::read_from_path(path).unwrap();
        let item_id = utils::import_song(&self.collection, &metadata.tags);
        if let Some(item_id) = item_id {
            self.collection.music().add_source_file(item_id, SourceType::LocalFile, String::from(path.to_str().unwrap()));
            logger.log_song(&metadata.tags);
        }
    }
}

struct ImportTask {
    root_folder: PathBuf,
    log_path: Option<PathBuf>,
    importers: HashMap<String, Box<dyn Importer>>,
}

impl ImportTask {

    pub fn new(collection: Service<Collection>, path: PathBuf, log_path: Option<PathBuf>) -> Self {
        let mut importers = HashMap::<String, Box<dyn Importer>>::new();

        let audio_importer = Box::new(AudioImporter {
            collection: collection.clone()
        });
        importers.insert("mp3".to_string(), audio_importer.clone());

        Self {
            root_folder: path,
            log_path,
            importers,
        }
    }

    pub fn run(&self, _: &TaskContext) {
        log::debug!("Import start");

        let mut logger: Box<dyn ImportLogger> = match self.log_path.as_ref() {
            Some(log_path) => {
                Box::new(CsvLogger::new(log_path))
            },
            None => {
                Box::new(DummyLogger::new())
            }
        };

        for entry in WalkDir::new(&self.root_folder) {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                self.import_file(entry.path(), logger.as_mut());
            }
        }
    }

    fn import_file(&self, path: &Path, logger: &mut dyn ImportLogger) {
        println!("{}", path.display());

        let extension = path.extension().unwrap().to_str().unwrap();
        match self.importers.get(extension) {
            Some(importer) => {
                importer.import(path, logger);
            },
            None => {}
        }
    }

}

pub struct CollectionImporter {
    task_manager: Service<TaskManager>,
    collection: Service<Collection>,
    log_path: PathBuf,
}

impl CollectionImporter {

    pub fn import(&self, path: PathBuf, create_log: bool) {
        let log_path = if create_log {
            Some(self.log_path.clone())
        } else {
            None
        };
        let task = ImportTask::new(self.collection.clone(), path, log_path);
        self.task_manager.run_instant_task(move |task_feedback| {
            task.run(task_feedback);
        })
    }

    pub fn import_basic(&self, tags: HashMap<String, String>, file_path: String) {
        let item_id = utils::import_song(&self.collection, &TagsMap::from_map(tags));
        if let Some(item_id) = item_id {
            self.collection.music().add_source_file(item_id, SourceType::LocalFile, file_path);
        }
    }

}

impl ServiceApi for CollectionImporter {

}

impl ServiceInitializer for CollectionImporter {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let cmd_manager = context.get_service::<CmdManager>();
        let collection = context.get_service::<Collection>();
        let platform_api = context.get_service::<PlatformApi>();

        let mut log_path = platform_api.file_system.get_workspace_dir();
        log_path.push("import.log");

        let importer = Arc::new(CollectionImporter {
            task_manager: context.get_service(),
            collection,
            log_path,
        });

        register_rpc_handler!(rpc, importer, "lappi.import.import_basic", import_basic(tags: HashMap<String, String>, file_path: String));

        let import_collection_cmd_description = CmdDescription {
            call_name: "import.collection".to_string(),
            description: Some("Init collection from csv files".to_string()),
            args: HashMap::from([
                ("dir".to_string(), ArgDescription {
                    call_name: "dir".to_string(),
                    description: Some("Root directory".to_string()),
                    arg_type: ArgType::STRING,
                }),
                ("create_log".to_string(), ArgDescription {
                    call_name: "create_log".to_string(),
                    description: Some("Create import log".to_string()),
                    arg_type: ArgType::BOOL,
                }),
            ]),
        };
        let importer_copy = importer.clone();
        cmd_manager.add_command(import_collection_cmd_description, move |args| {
            let path_str = args.get_string("dir");
            let path = PathBuf::from(path_str);
            let create_log = args.get_bool("create_log");
            importer_copy.import(path, create_log);
        });

        return importer;
    }
}
