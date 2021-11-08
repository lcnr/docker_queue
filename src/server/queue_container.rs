use super::ServerError;
use crate::domain::QueuedContainer;
use anyhow::Context;
use axum::{extract::Extension, Json};
use tokio::sync::mpsc::Sender;

#[tracing::instrument(name = "Queue container", skip(tx))]
pub async fn queue_container(
    Json(queued_container): Json<QueuedContainer>,
    Extension(tx): Extension<Sender<QueuedContainer>>,
) -> Result<(), ServerError> {
    tx.send(queued_container)
        .await
        .context("Receiver dropped.")?;
    Ok(())
}
