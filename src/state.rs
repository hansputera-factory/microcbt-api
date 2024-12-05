use once_cell::sync::Lazy;

use crate::config::config;

pub static CONFIG_STATES: Lazy<config::ConfigStruct> = Lazy::new(|| config::ConfigStruct::new().expect("Error reading configuration"));