use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::sync::Arc;

use amina_core::service::{Context, ServiceApi, ServiceInitializer};
use amina_core::settings::{Settings as SettingsEntry, SettingsManager};

use crate::platform_api::PlatformApi;


#[derive(Clone)]
pub struct Settings {
    entry: Arc<SettingsEntry>,
}

impl Deref for Settings {
     type Target = SettingsEntry;

     fn deref(&self) -> &Self::Target {
         &self.entry
     }
}

impl ServiceApi for Settings {

    fn start(&self) {
        self.get_string("test.string.prop");
    }

    fn stop(&self) {
        self.save_to_file();
    }

}

impl ServiceInitializer for Settings {

    fn initialize(context: &Context) -> Arc<Self> {
        let settings_manager = context.get_service::<SettingsManager>();
        let platform = context.get_service::<PlatformApi>();

        let path = platform.file_system.get_workspace_dir().join("settings.yaml");

        let settings_entry = if path.exists() {
            let mut file = File::open(path.as_path()).unwrap();
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();
            SettingsEntry::init_from_string(&buffer, path.as_path())
        } else {
            SettingsEntry::create_empty(path.as_path())
        };
        let settings_entry = Arc::new(settings_entry);
        let settings = Arc::new(Settings {
            entry: settings_entry.clone(),
        });

        settings_manager.register_settings(settings_entry);

        return settings;
    }

}
