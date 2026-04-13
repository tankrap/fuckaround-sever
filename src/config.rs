use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Config
{
    pub postgres: String,
    pub log_level: Option<String>,
    pub migrations: String,
    pub port: u16,
    pub host: String,
    pub redis: String
}
