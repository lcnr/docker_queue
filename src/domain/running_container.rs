use bollard::models::ContainerSummaryInner;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum RunningContainer {
    Tracked(ContainerSummaryInner),
    External(ContainerSummaryInner),
}
