use super::State;
use crate::{domain::Container, server::ServerError};
use anyhow::Result;
use axum::{extract::Extension, Json};
use bollard::{container::ListContainersOptions, models::ContainerSummaryInner, Docker};
use std::{collections::HashMap, sync::Arc};

#[tracing::instrument(name = "List containers", skip(state))]
pub(super) async fn list_containers(
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Vec<Container>>, ServerError> {
    let containers = state.get_containers().await?;
    Ok(Json(containers))
}

impl State {
    pub(super) async fn get_containers(&self) -> Result<Vec<Container>> {
        let mut containers = get_running_containers()
            .await?
            .into_iter()
            .map(|container| Container::Running(Box::new(container)))
            .collect::<Vec<_>>();
        let mut queued_containers = { self.queued_containers.lock().unwrap().clone() }
            .into_iter()
            .map(Container::Queued)
            .collect::<Vec<_>>();
        containers.append(&mut queued_containers);
        Ok(containers)
    }
}

pub async fn get_running_containers() -> Result<Vec<ContainerSummaryInner>> {
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
        .map(|container| container)
        .collect::<Vec<_>>();

    Ok(containers)
}
