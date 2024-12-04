use config::Config;


#[derive(serde::Deserialize)]
pub struct ConfigLoggerStruct {
    pub format: String,
    pub path: String,
}

#[derive(serde::Deserialize)]
pub struct ConfigStruct {
    pub postgre_url: String,
    pub listen_host: String,
    pub listen_port: u16,
    pub logger: ConfigLoggerStruct,
}

pub fn read_config(file_config: &str) -> ConfigStruct {
    let settings_result = Config::builder()
        .add_source(config::File::with_name(file_config))
        .build();

    let settings = match settings_result {
        Ok(settings) => settings,
        Err(e) => panic!("Error reading configuration: {}", e)
    };

    settings.try_deserialize::<ConfigStruct>().unwrap()
}