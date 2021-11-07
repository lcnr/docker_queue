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
    path: String, // TODO: use correct path struct
    command: String,
    queued_at: String, // TODO: use chrono or any time struct
}

impl QueuedContainer {
    pub fn from_command(_command: String) -> Result<Self> {
        todo!()
    }

    pub async fn queue(&self) -> Result<Container> {
        todo!()
    }

    // async fn execute(&self) -> Result<()> {
    //     todo!()
    // }
}
