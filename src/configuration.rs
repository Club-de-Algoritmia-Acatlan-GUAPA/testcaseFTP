use anyhow::Result;
use config::{Config, FileFormat};
use dotenvy::dotenv;

const CONFIGURATION_DIRECTORY: &str = "CONFIGURATION_DIRECTORY";
const CONFIGURATION_FILE: &str = "CONFIGURATION_FILE";

#[derive(serde::Deserialize, Clone)]
pub struct FtpSettings {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub ftp: FtpSettings,
    pub ftp_home: String,
    pub is_prod: bool,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let (is_prod, config_dir, config_file);
    match dotenv() {
        Ok(_) => {
            is_prod = dotenvy::var("IS_PROD")
                .expect("IS_PROD is not set")
                .parse::<bool>()
                .unwrap();
            config_dir =
                dotenvy::var(CONFIGURATION_DIRECTORY).expect("CONFIGURATION_DIRECTORY is not set");
            config_file = dotenvy::var(CONFIGURATION_FILE).expect("CONFIGURATION_FILE is not set");
        },
        Err(_) => {
            is_prod = std::env::var("IS_PROD")
                .expect("IS_PROD is not set")
                .parse::<bool>()
                .unwrap();
            config_dir =
                std::env::var(CONFIGURATION_DIRECTORY).expect("CONFIGURATION_DIRECTORY is not set");
            config_file = std::env::var(CONFIGURATION_FILE).expect("CONFIGURATION_FILE is not set");
        },
    }

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join(config_dir);

    let settings = Config::builder()
        .add_source(
            config::File::from(configuration_directory.join(config_file)).format(FileFormat::Yaml),
        )
        .build()?;

    settings
        .try_deserialize::<Settings>()
        .map(|s| Settings { is_prod, ..s })
}
