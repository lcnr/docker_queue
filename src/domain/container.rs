use super::QueuedContainer;
use bollard::models::ContainerSummaryInner;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Container {
    Running(Box<ContainerSummaryInner>),
    Queued(QueuedContainer),
}
