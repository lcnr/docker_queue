use super::{ServerError, State, TaskMessage};
use crate::domain::QueuedContainer;
use anyhow::Context;
use axum::{extract::Extension, Json};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

#[tracing::instrument(name = "Queue container", skip(state))]
pub(super) async fn queue_container(
    Json(queued_container): Json<QueuedContainer>,
    Extension(state): Extension<Arc<State>>,
    Extension(tx): Extension<Sender<TaskMessage>>,
) -> Result<(), ServerError> {
    let check_run = queued_container.is_queued();
    state
        .queued_containers
        .lock()
        .unwrap()
        .push(queued_container);
    if check_run {
        tx.send(TaskMessage::CheckRun)
            .await
            .context("Receiver dropped.")?;
    }
    Ok(())
}
