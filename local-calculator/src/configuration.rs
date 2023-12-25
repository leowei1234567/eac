#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub build_addresses_from_file: bool,
    pub address_file_path: String,
    pub thread_count: u8,
    pub sleep_each_round: bool,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("local-calculator/configuration");

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("config.yaml"),
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
