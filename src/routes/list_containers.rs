use crate::{domain::Container, server::ServerError};
use anyhow::Result;
use axum::Json;
use bollard::{container::ListContainersOptions, Docker};
use std::collections::HashMap;

#[tracing::instrument(name = "List containers")]
pub async fn list_containers() -> Result<Json<Vec<Container>>, ServerError> {
    let containers = get_containers().await?;
    Ok(Json(containers))
}

async fn get_containers() -> Result<Vec<Container>> {
    let docker = Docker::connect_with_local_defaults()?;
    let filters = HashMap::from([("status", vec!["running"])]);
    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });
    let containers = docker
        .list_containers(options)
        .await?
        .into_iter()
        .map(|container| Container::Running(Box::new(container)))
        .collect::<Vec<_>>();
    Ok(containers)
}
