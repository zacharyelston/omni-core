//! Application services

mod session;

use crate::config::Config;
use std::sync::Arc;

pub use session::{Session, SessionStore};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub sessions: SessionStore,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            sessions: SessionStore::new(),
        }
    }
}
