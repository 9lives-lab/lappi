use std::path::PathBuf;
use std::sync::Arc;
use directories::{ProjectDirs};
use lappi_core::platform_api::FileSystemApi;

pub struct DesktopFileSystemApi {
    workspace_dir: PathBuf,
}

impl FileSystemApi for DesktopFileSystemApi {

    fn get_workspace_dir(&self) -> PathBuf {
        self.workspace_dir.clone()
    }

}

pub fn initialize() -> Arc<DesktopFileSystemApi> {
    let working_dir = std::env::current_dir().unwrap();
    log::debug!("Working dir: {:?}", working_dir);

    let workspace_dir = if cfg!(debug_assertions) {
        let workspace = std::env::var("LAPPI_WORKSPACE").unwrap_or("default".to_string());
        let workspace_dir = workspace.replace('.', "/");
        working_dir.join("lappi_lab/debug_workspace").join(workspace_dir)
    } else {
        // Lin: /home/alice/.config/lappi
        // Win: C:\Users\Alice\AppData\Roaming\Lappi\config
        // Mac: /Users/Alice/Library/Application Support/Lappi
        let proj_dirs = ProjectDirs::from("", "", "Lappi").unwrap();
        proj_dirs.config_dir().to_path_buf()
    };

    log::info!("Workspace dir: {:?}", workspace_dir);

    let api = DesktopFileSystemApi {
        workspace_dir
    };
    return Arc::new(api);
}
