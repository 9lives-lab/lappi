use serde::Deserialize;

pub mod database {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub enum Mode {
        RAM,
        FILE
    }

    #[derive(Deserialize)]
    pub struct SqliteConfig {
        pub mode: Mode,
    }

    #[derive(Deserialize)]
    pub struct Config {
        pub sqlite_config: SqliteConfig,
    }

}

pub mod collection {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Config {
        pub init: bool,
        pub init_folder: String,
        pub storage: bool,
    }

}

#[derive(Deserialize)]
pub struct DebugConfig {
    pub database: self::database::Config,
    pub collection: self::collection::Config,
}

