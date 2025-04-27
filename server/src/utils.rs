use std::str::FromStr;

use log::{error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::BufReader};

pub static SOLANA_RPC: Lazy<String> = Lazy::new(|| env::var("SOLANA_RPC").unwrap_or_default());
pub static HEURIST_MESH_URL: Lazy<String> =
    Lazy::new(|| env::var("HEURIST_MESH_URL").unwrap_or_default());
pub static HEURIST_API_KEY: Lazy<String> =
    Lazy::new(|| env::var("HEURIST_API_KEY").unwrap_or_default());

/// Global static collection of LP wallet addresses
pub static LP_WALLETS: Lazy<Vec<String>> = Lazy::new(|| read_lp_wallets_config("config.json"));
/// Configuration structure for LP wallets
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    lp_wallets: Vec<String>,
}

/// Reads LP wallet addresses from the configuration file
fn read_lp_wallets_config(config_path: &str) -> Vec<String> {
    match File::open(config_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match serde_json::from_reader::<_, Config>(reader) {
                Ok(config) => {
                    info!("Loaded {} LP wallets from config", config.lp_wallets.len());
                    config.lp_wallets
                }
                Err(e) => {
                    error!("Error parsing config file: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            error!("Error opening config file: {}", e);
            Vec::new()
        }
    }
}

pub fn read_parse_env<T: Clone + FromStr>(var_name: &'static str) -> T {
    let var_value =
        std::env::var(var_name).unwrap_or_else(|_| panic!("Can not read env of {}", var_name));
    match var_value.parse::<T>() {
        Ok(v) => v,
        Err(_) => panic!("Cannot parse {} env var", var_name),
    }
}
