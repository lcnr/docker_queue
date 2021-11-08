use crate::error_chain_fmt;
use bollard::models::ContainerSummaryInner;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(thiserror::Error)]
pub enum ContainerError {
    #[error("Invalid docker run command: {0}")]
    InvalidQueuedCommand(String),
}

impl std::fmt::Debug for ContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueuedContainer {
    id: Uuid,
    command: String,
    status: QueuedContainerStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueuedContainerStatus {
    Queued,
    Paused,
}

impl std::fmt::Display for QueuedContainerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            QueuedContainerStatus::Queued => "Queued",
            QueuedContainerStatus::Paused => "Paused",
        };
        write!(f, "{}", s)
    }
}

impl QueuedContainer {
    pub fn new(command: impl Into<String>, paused: bool) -> Result<Self, ContainerError> {
        let id = Uuid::new_v4();
        let command = command.into();
        if !command.starts_with("docker run") {
            return Err(ContainerError::InvalidQueuedCommand(command));
        }

        let status = if paused {
            QueuedContainerStatus::Paused
        } else {
            QueuedContainerStatus::Queued
        };
        Ok(Self {
            id,
            command,
            status,
        })
    }

    /// Get a reference to the queued container's id.
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    /// Get a reference to the queued container's command.
    pub fn command(&self) -> &str {
        self.command.as_ref()
    }

    /// Get a reference to the queued container's status.
    pub fn status(&self) -> &QueuedContainerStatus {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn reject_queued_containers_with_invalid_command() {
        let valid_cmd = "docker run some_image";
        assert_ok!(QueuedContainer::new(valid_cmd, false));
        let invalid_cmd = "docker lalala";
        assert_err!(QueuedContainer::new(invalid_cmd, false));
    }
}
