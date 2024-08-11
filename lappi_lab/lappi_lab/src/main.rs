mod platform_impl;

use std::ops::Deref;

use simple_logger::SimpleLogger;
use amina_core::service::Context;
use amina_core::events::EventEmitter;
use amina_core::rpc::Rpc;
use amina_core::tasks::TaskManager;
use amina_core::cmd_manager::CmdManager;
use amina_core::cmd_manager::cli_adapter::CmdManagerCliAdapter;
use amina_core::settings::SettingsManager;
use amina_server::rpc_web_gate::RpcServer;

use lappi_core::platform_api::PlatformApi;
use lappi_core::debug::Debugger;
use lappi_core::collection::Collection;
use lappi_core::collection::storage::local::LocalStorage;
use lappi_core::playback::Playback;
use lappi_core::database::Database;
use lappi_core::exploring::chat::ChatService;
use lappi_core::exploring::chat::templates::ChatTemplates;
use lappi_core::import::collection::CollectionImporter;
use lappi_core::settings::Settings;
use lappi_core::exploring::lyrics::LyricsExplorer;
use lappi_core::file_manager::FileManager;
use lappi_core::file_manager::search::FilesExplorer;
use lappi_core::playlists::classic::ClassicPlaylists;
use lappi_core::py_server_client::PyServerClient;

fn main() {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Debug)
        .with_module_level("hyper", log::LevelFilter::Info)
        .with_module_level("reqwest", log::LevelFilter::Info)
        .init().unwrap();

    log::info!("Lappi Lab");

    let file_system_api = platform_impl::file_system::initialize();

    let platform_api = PlatformApi {
        file_system: file_system_api,
    };

    let context = Context::new();

    context.add_service(platform_api);
    context.init_service::<Debugger>();
    context.init_service::<TaskManager>();
    context.init_service::<EventEmitter>();
    context.init_service::<Rpc>();
    context.init_service::<CmdManager>();
    context.init_service::<SettingsManager>();
    context.init_service::<Settings>();
    context.init_service::<FileManager>();
    context.init_service::<FilesExplorer>();
    context.init_service::<Database>();
    context.init_service::<LocalStorage>();
    context.init_service::<Collection>();
    context.init_service::<ClassicPlaylists>();
    context.init_service::<Playback>();
    context.init_service::<CollectionImporter>();
    context.init_service::<PyServerClient>();
    context.init_service::<ChatService>();
    context.init_service::<ChatTemplates>();
    context.init_service::<LyricsExplorer>();

    context.start();

    log::debug!("Core initializing complete");

    let server = RpcServer::run(&context);

    let cmd_manager = context.get_service::<CmdManager>();
    let cli_adapter = CmdManagerCliAdapter::new(cmd_manager.deref());

    log::info!("Initializing complete");

    cli_adapter.run();

    log::info!("Closing application");

    server.stop();

    context.stop();
}