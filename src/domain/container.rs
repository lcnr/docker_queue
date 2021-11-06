use bollard::models::ContainerSummaryInner;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Container {
    Ignored(IgnoredContainer),
    Running(Box<ContainerSummaryInner>),
    Queued(QueuedContainer),
}

// impl Container {
//     fn show(&self) {
//         todo!()
//     }
// }

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

// impl QueuedContainer {
//     fn from_command(command: String) -> Result<Self> {
//         todo!()
//     }

//     async fn queue(&self) -> Result<Container> {
//         todo!()
//     }

//     async fn execute(&self) -> Result<()> {
//         todo!()
//     }
// }

// pub async fn list_containers() -> Result<()> {
//     let containers = get_containers().await?;
//     containers
//         .into_iter()
//         .for_each(|container| container.show());
//     Ok(())
// }

// pub async fn queue_container(command: String) -> Result<()> {
//     let container = QueuedContainer::from_command(command)?;
//     match container.queue().await? {
//         Container::Running(container) => {
//             // some information about the container being able to run
//         }
//         Container::Queued(container) => {
//             // some information about the container being queued
//         }
//         _ => {}
//     }
//     list_containers().await?;
//     Ok(())
// }
