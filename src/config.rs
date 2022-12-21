use super::error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct DConfigWorkspace {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DConfig {
    version: i64,
    workspace: DConfigWorkspace,
}

impl DConfig {
    pub fn new(path: &PathBuf) -> crate::Result<Self> {
        fs::read_to_string(path.join(super::constants::DENDRON_CONFIG_FILE))
            .map(|value| {
                let d_config: DConfig = serde_yaml::from_str(&value.to_string()).unwrap();
                return d_config;
            })
            .map_err(error::DendronError::Io)
    }
}
