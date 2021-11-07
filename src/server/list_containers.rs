use crate::{
    domain::{get_containers, Container},
    server::ServerError,
};
use anyhow::Result;
use axum::Json;

#[tracing::instrument(name = "List containers")]
pub async fn list_containers() -> Result<Json<Vec<Container>>, ServerError> {
    let containers = get_containers().await?;
    Ok(Json(containers))
}
