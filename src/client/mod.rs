mod get_running_container;
mod list_containers;
mod queue_container;

use crate::error_chain_fmt;
use axum::http::StatusCode;

pub struct ClientApp<W: std::io::Write> {
    pub port: u16,
    pub writer: W,
}

impl<W: std::io::Write> ClientApp<W> {
    pub fn new(port: u16, writer: W) -> Self {
        Self { port, writer }
    }
}

#[derive(thiserror::Error)]
pub enum ClientError {
    #[error("Unexpected status received: {0}")]
    ServerStatusError(StatusCode),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
