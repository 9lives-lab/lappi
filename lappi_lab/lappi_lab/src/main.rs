mod platform_impl;

use std::net::SocketAddr;
use std::path::PathBuf;

use amina_core::events::EventEmitter;
use amina_core::rpc::Rpc;
use amina_core::service::Context;
use amina_core::tasks::TaskManager;
use amina_core::cmd_manager::CmdManager;
use amina_core::settings::SettingsManager;
use amina_server::cli::SimpleCliContext;
use amina_server::rpc_web_gate::RpcServer;
use amina_server::rpc_web_gate::RpcServerConfig;
use amina_server::cli::adapters::cmd_manager_adapter::CmdManagerAdapter;

use lappi_core::platform_api::PlatformApi;
use lappi_core::platform_api::FileSystemApi;
use lappi_core::app_config::AppConfig;
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
    context.init_service::<SettingsManager>();

    let cli_history_file = file_system_api.get_workspace_dir().join("cli_history.txt");
    let cmd_manager = context.get_service::<CmdManager>();
    let cli_adapter = CmdManagerAdapter::new(cmd_manager);
    let cli_filters = vec![
        ("hyper".to_string(), LevelFilter::Info),
        ("reqwest".to_string(), LevelFilter::Info)
    ];
    let mut cli_context = SimpleCliContext::create(
        Box::new(cli_adapter), 
        cli_filters,
        cli_history_file.as_std_path()
    );

    log::info!("Lappi Lab");

    lappi_core::initilaize();

    context.start();

    log::debug!("Core initializing complete");

    let server = start_rpc_server(context);

    log::info!("Initializing complete");

    cli_context.run();

    log::info!("Closing application");

    server.stop();

    context.stop();
}

fn start_rpc_server(context: &Context) -> RpcServer {
    let app_config = context.get_service::<AppConfig>();

    let static_files_path = if app_config.web_server.static_files_path.is_empty() {
        None
    } else {
        Some(PathBuf::from(&app_config.web_server.static_files_path))
    };

    let rpc_server_config = RpcServerConfig {
        socket_address: SocketAddr::from(([0, 0, 0, 0], app_config.web_server.port)),
        static_files_path,
    };

    return RpcServer::run(context, &rpc_server_config);
}
