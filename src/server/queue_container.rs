use super::State;
use crate::domain::QueuedContainer;
use axum::{extract::Extension, Json};
use std::sync::Arc;

#[tracing::instrument(name = "Queue container", skip(state))]
pub async fn queue_container(
    Json(queued_container): Json<QueuedContainer>,
    Extension(state): Extension<Arc<State>>,
) {
    state
        .queued_containers
        .lock()
        .unwrap()
        .push(queued_container);
}
