use crate::{
    domain::{Container, QueuedContainer},
    server::ServerError,
};
use anyhow::Result;
use axum::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueueRequest {}

#[tracing::instrument(name = "Queue container")]
pub async fn queue_container(
    Json(queue_request): Json<QueueRequest>,
) -> Result<Json<Container>, ServerError> {
    let container = QueuedContainer::from_command("asdasd".into())?
        .queue()
        .await?;
    Ok(Json(container))
}
