use super::{QueuedContainer, RunningContainer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Container {
    Running(Box<RunningContainer>),
    Queued(QueuedContainer),
}
