use config::{Config, ConfigError};


#[derive(serde::Deserialize, Debug, Clone)]
pub struct ConfigLoggerStruct {
    pub format: String,
    pub path: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ConfigAuthStruct {
    pub limit: i64,
    pub tolerance_different: i64,
    pub block_different_device: bool,
    pub block_different_useragent: bool
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ConfigStruct {
    pub postgre_url: String,
    pub listen_host: String,
    pub listen_port: u16,
    pub jwt_secret: String,
    pub logger: ConfigLoggerStruct,
    pub auth: ConfigAuthStruct
}

impl ConfigStruct {
    pub fn new() -> Result<Self, ConfigError> {
        let settings_result = Config::builder()
        .add_source(config::File::with_name("config.yaml"))
        .build();

        let settings = match settings_result {
            Ok(settings) => settings,
            Err(e) => panic!("Error reading configuration: {}", e)
        };

        settings.try_deserialize::<ConfigStruct>()    
    }
}
