use toml;
use std::fs::read_to_string;
use serde::Deserialize;
use super::common;

#[derive(Debug, Deserialize)]
pub struct FocusConfig {
    pub version: String,
    pub hosts_file: String,
    pub socket_file: String,
    pub blocked: Vec<String>
}

#[derive(Debug)]
pub enum FocusConfigError {
    ConfigMissing,
    ConfigInvalid
}

pub fn read_config(configfile: &String) -> Result<FocusConfig, FocusConfigError> {
    if !common::file_exists(&configfile) {
        return Err(FocusConfigError::ConfigMissing);
    }
    let config_string = read_to_string(&configfile);

    let config: FocusConfig = match  toml::from_str(&config_string.unwrap()) {
        Ok(cfg) => cfg,
        Err(_)  => return Err(FocusConfigError::ConfigInvalid)
    };
    return Ok(config)
}

