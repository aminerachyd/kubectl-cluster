use std::fs;
use std::io::{self, ErrorKind, Read, Write};

use serde::{Deserialize, Serialize};

use super::cluster::Cluster;

#[derive(Serialize, Deserialize)]
pub struct CliConfig {
    pub clusters: Vec<Cluster>,
}

/**
 * Creates config under ~/.config/kubectl-cluster/clusters
 */
pub fn create_config() -> Result<CliConfig, io::Error> {
    let dir_create = fs::create_dir(get_config_dir());

    if let Err(error) = dir_create {
        if error.kind() != ErrorKind::AlreadyExists {
            return Err(io::Error::new(
                ErrorKind::Other,
                "Error opening config file, exiting...",
            ));
        }
    }

    let mut file = fs::File::create(get_config_file())?;

    let cli_config = serde_yaml::to_string(&default_config()).unwrap();

    file.write(cli_config.as_bytes())?;

    Ok(default_config())
}

/**
 * Reads config from config file
 * If it doesn't exist, it creates it
 */
pub fn read_config() -> Result<CliConfig, io::Error> {
    let mut cli_config = String::new();

    let file_open_result = fs::File::open(get_config_file());

    match file_open_result {
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                return create_config();
            } else {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    "Error opening config file, exiting...",
                ));
            }
        }

        Ok(mut file) => {
            file.read_to_string(&mut cli_config)?;

            let config_parse = serde_yaml::from_str::<CliConfig>(&cli_config);
            match config_parse {
                Err(_) => {
                    return Err(io::Error::new(
                        ErrorKind::InvalidData,
                        "Error parsing the config file",
                    ))
                }
                Ok(cli_config) => return Ok(cli_config),
            }
        }
    }
}

/**
 * Returns path of config dir
 */
fn get_config_dir() -> String {
    let home = env!("HOME");

    format!("{home}/.config/kubectl-cluster")
}

/**
 * Returns path of config file
 */
fn get_config_file() -> String {
    format!("{}/clusters", get_config_dir())
}

/**
 * Generates a default config
 */
fn default_config() -> CliConfig {
    CliConfig { clusters: vec![] }
}
