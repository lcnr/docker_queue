use crate::domain::QueuedContainer;
use axum::{http::StatusCode, Json};

#[tracing::instrument(name = "Queue container")]
pub async fn queue_container(Json(queued_container): Json<QueuedContainer>) -> StatusCode {
    StatusCode::OK
}
