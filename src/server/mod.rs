mod launcher_task;
mod list_containers;
mod queue_container;
mod startup;

pub(self) use launcher_task::*;
pub(self) use list_containers::*;
pub(self) use queue_container::*;
pub use startup::*;

use crate::domain::QueuedContainer;
use std::sync::Mutex;

struct State {
    queued_containers: Mutex<Vec<QueuedContainer>>,
}

impl State {
    fn new() -> Self {
        Self {
            queued_containers: Mutex::new(Vec::new()),
        }
    }
}
