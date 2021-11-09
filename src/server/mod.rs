mod launcher_task;
mod list_containers;
mod queue_container;
mod startup;

pub(self) use launcher_task::*;
pub(self) use list_containers::*;
pub(self) use queue_container::*;
pub use startup::*;

use crate::domain::QueuedContainer;
use crate::error_chain_fmt;
use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::convert::Infallible;
use std::sync::Mutex;

struct State {
    queued_containers: Mutex<Vec<QueuedContainer>>,
}

impl State {
    fn new() -> Self {
        Self {
            queued_containers: Mutex::new(Vec::new()),
        }
    }
}

#[derive(thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl IntoResponse for ServerError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_message) = match self {
            ServerError::UnexpectedError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
