use bollard::models::ContainerSummaryInner;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Container {
    // Ignored(IgnoredContainer),
    Running(Box<ContainerSummaryInner>),
    Queued(QueuedContainer),
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct IgnoredContainer {
//     name: String,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueuedContainer {
    pub name: String,
}

impl QueuedContainer {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
