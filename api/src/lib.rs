use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::broadcast;
use shared::WsEvent;

pub mod config;
pub mod db;
pub mod auth;
pub mod errors;
pub mod handlers;
pub mod middleware;

pub type AppState = Arc<InnerState>;

pub struct InnerState {
    pub db: PgPool,
    pub config: config::Config,
    pub ws_tx: broadcast::Sender<WsEvent>,
}
