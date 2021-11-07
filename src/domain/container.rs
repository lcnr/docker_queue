use std::collections::HashMap;

use anyhow::Result;
use bollard::{container::ListContainersOptions, models::ContainerSummaryInner, Docker};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Container {
    Ignored(IgnoredContainer),
    Running(Box<ContainerSummaryInner>),
    Queued(QueuedContainer),
}

pub async fn get_containers() -> Result<Vec<Container>> {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct IgnoredContainer {
    name: String,
}

// fn get_ignored_containers() -> Vec<IgnoredContainer> {
//     todo!()
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct QueuedContainer {
    name: String,
}

impl QueuedContainer {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
