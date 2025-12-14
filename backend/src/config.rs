//! Server configuration

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_secret_key")]
    pub secret_key: String,

    #[serde(default = "default_session_ttl")]
    pub session_ttl_secs: u64,
}

fn default_port() -> u16 {
    8080
}

fn default_secret_key() -> String {
    "change-me-in-production".to_string()
}

fn default_session_ttl() -> u64 {
    3600 // 1 hour
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or_else(default_port),
            secret_key: std::env::var("SECRET_KEY")
                .unwrap_or_else(|_| default_secret_key()),
            session_ttl_secs: std::env::var("SESSION_TTL")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or_else(default_session_ttl),
        })
    }
}
