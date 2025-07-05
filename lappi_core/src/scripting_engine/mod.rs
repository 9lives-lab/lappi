use std::process::Command;
use std::sync::Arc;

use camino::Utf8PathBuf;
use rhai::{Dynamic, Engine, Scope};
use serde::Serialize;
use amina_core::register_rpc_handler;
use amina_core::rpc::{Rpc, RpcGate};
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};

use crate::collection::music::MusicItemId;
use crate::platform_api::PlatformApi;

#[derive(Serialize)]
pub struct FilesList {
    pub folders: Vec<String>,
    pub files: Vec<String>,
}

pub struct ScriptingEngine {
    platform_api: Service<PlatformApi>,
    rpc_gate: Service<RpcGate>,
}

impl ScriptingEngine {
    pub fn get_scripts_list(&self) -> Vec<String> {
        let mut scripts_list = Vec::new();

        if let Ok(dir_entries) = self.get_scripts_folder().read_dir() {
            for entry in dir_entries {
                if let Ok(entry) = entry {
                    if entry.file_type().map_or(false, |file_type| file_type.is_file()) {
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext.eq("rhai")) {
                            if let Some(file_name) = path.file_name().and_then(|p| p.to_str()) {
                                scripts_list.push(file_name.to_string());
                            }
                        }
                    }
                }
            }
        };

        scripts_list
    }

    pub fn run(&self, script_name: &str, scope: &mut Scope) {
        log::info!("Run script: {}", script_name);

        let mut engine = Engine::new();

        let rpc_gate = self.rpc_gate.clone();
        engine.register_fn("send_rpc_request", move |key, args: rhai::Map| {
            let json_req = serde_json::to_string(&args).unwrap();
            let resp = rpc_gate.call_raw(key, &json_req);
            return serde_json::from_str::<Dynamic>(&resp).unwrap();
        });

        engine.register_fn("run_os_cmd", move |command: &str, args: rhai::Array| {
            let args: Vec<String> = args.iter().map(|val| val.to_string()).collect();
            return Command::new(command)
                .args(args)
                .status()
                .is_ok();
        });        
        
        engine.on_print(|msg| log::info!("{}", msg));

        let scripts_folder = self.get_scripts_folder();
        let path = scripts_folder.join(script_name).into_std_path_buf();
        let cmd_result = engine.run_file_with_scope(scope, path);
        log::error!("Script failed: {:?}", cmd_result);
    }

    pub fn run_for_music_item(&self, script_name: String, music_item_id: MusicItemId) {
        let mut scope = Scope::new();
        scope.push_constant("MUSIC_ITEM_ID", music_item_id);
        self.run(&script_name, &mut scope);
    }

    fn get_scripts_folder(&self) -> Utf8PathBuf {
        self.platform_api.file_system.get_workspace_dir().join("scripts")
    }
}

impl ServiceApi for ScriptingEngine {

}

impl ServiceInitializer for ScriptingEngine {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let rpc_gate = context.get_service::<RpcGate>();
        let platform_api = context.get_service::<PlatformApi>();

        let scripting_engine = Arc::new(Self {
            platform_api,
            rpc_gate
        });

        register_rpc_handler!(rpc, scripting_engine, "lappi.scripting_engine.get_scripts_list", get_scripts_list());
        register_rpc_handler!(rpc, scripting_engine, "lappi.scripting_engine.run_for_music_item", run_for_music_item(script_name: String, music_item_id: MusicItemId));

        return scripting_engine;
    }
}
