mod platform_impl;

use amina_core::events::EventEmitter;
use amina_core::rpc::Rpc;
use amina_core::tasks::TaskManager;
use amina_core::cmd_manager::CmdManager;
use amina_core::settings::SettingsManager;
use amina_server::cli::SimpleCliContext;
use amina_server::rpc_web_gate::RpcServer;
use amina_server::cli::adapters::cmd_manager_adapter::CmdManagerAdapter;

use lappi_core::platform_api::PlatformApi;
use lappi_core::platform_api::FileSystemApi;
use lappi_core::app_config::AppConfig;
use lappi_core::collection::Collection;
use lappi_core::collection::storage::local::LocalStorage;
use lappi_core::playback::players::web_player::WebPlayerService;
use lappi_core::playback::Playback;
use lappi_core::database::Database;
use lappi_core::exploring::chat::ChatService;
use lappi_core::exploring::chat::templates::ChatTemplates;
use lappi_core::import::collection::CollectionImporter;
use lappi_core::scripting_engine::ScriptingEngine;
use lappi_core::settings::Settings;
use lappi_core::exploring::lyrics::LyricsExplorer;
use lappi_core::file_manager::FileManager;
use lappi_core::file_manager::search::FilesExplorer;
use lappi_core::py_server_client::PyServerClient;
use log::LevelFilter;

fn main() {
    let file_system_api = platform_impl::file_system::initialize();
    let playback_api = platform_impl::playback::initialize();

    let platform_api = PlatformApi {
        file_system: file_system_api.clone(),
        playback: playback_api,
    };

    let context = lappi_core::context();

    context.add_service(platform_api);
    context.init_service::<AppConfig>();
    context.init_service::<TaskManager>();
    context.init_service::<EventEmitter>();
    context.init_service::<Rpc>();
    context.init_service::<CmdManager>();

    let cli_history_file = file_system_api.get_workspace_dir().join("cli_history.txt");
    let cmd_manager = context.get_service::<CmdManager>();
    let cli_adapter = CmdManagerAdapter::new(cmd_manager);
    let cli_filters = vec![
        ("hyper".to_string(), LevelFilter::Info),
        ("reqwest".to_string(), LevelFilter::Info)
    ];
    let mut cli_context = SimpleCliContext::create(Box::new(cli_adapter), cli_filters, &cli_history_file);

    log::info!("Lappi Lab");

    context.init_service::<SettingsManager>();
    context.init_service::<Settings>();
    context.init_service::<ScriptingEngine>();
    context.init_service::<FileManager>();
    context.init_service::<FilesExplorer>();
    context.init_service::<Database>();
    context.init_service::<LocalStorage>();
    context.init_service::<Collection>();
    context.init_service::<WebPlayerService>();
    context.init_service::<Playback>();
    context.init_service::<CollectionImporter>();
    context.init_service::<PyServerClient>();
    context.init_service::<ChatService>();
    context.init_service::<ChatTemplates>();
    context.init_service::<LyricsExplorer>();

    context.start();

    log::debug!("Core initializing complete");

    let server = RpcServer::run(context);

    log::info!("Initializing complete");

    cli_context.run();

    log::info!("Closing application");

    server.stop();

    context.stop();
}
