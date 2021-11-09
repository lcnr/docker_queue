use crate::error_chain_fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(thiserror::Error)]
pub enum QueuedContainerError {
    #[error("Invalid docker run command: {0}")]
    InvalidQueuedCommand(String),
}

impl std::fmt::Debug for QueuedContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

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
    /// * `command` - A docker run command, should include a detach flag as "-d" or "--detach"
    pub fn new(command: impl Into<String>) -> Result<Self, QueuedContainerError> {
        let id = Uuid::new_v4();
        let command: String = command.into();
        if !command.starts_with("docker run") {
            return Err(QueuedContainerError::InvalidQueuedCommand(
                "Should start with \"docker run\"".into(),
            ));
        }
        let detach_flags = command
            .split_whitespace()
            .skip(2)
            .take_while(|x| x.starts_with('-'))
            .filter(|&x| (x == "-d") | (x == "--detach") | (x == "--detach=true"))
            .count();

        if detach_flags != 1 {
            return Err(QueuedContainerError::InvalidQueuedCommand(
                "Include a detach flag such as: \"-d\" or \"--detach\"".into(),
            ));
        }

        Ok(Self {
            id,
            command,
            status: QueuedContainerStatus::Paused,
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

    /// Set the queued container's status to `QueuedContainerStatus::Queued`.
    pub fn queue(&mut self) {
        self.status = QueuedContainerStatus::Queued;
    }

    /// Set the queued container's status to `QueuedContainerStatus::Paused`.
    pub fn pause(&mut self) {
        self.status = QueuedContainerStatus::Paused;
    }

    pub fn is_paused(&self) -> bool {
        self.status == QueuedContainerStatus::Paused
    }

    pub fn is_queued(&self) -> bool {
        self.status == QueuedContainerStatus::Queued
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn reject_queued_containers_without_run() {
        // Valid commands
        assert_ok!(QueuedContainer::new("docker run -d some_image"));
        // Invalid commands
        assert_err!(QueuedContainer::new("docker lalala"));
    }

    #[test]
    fn reject_queued_containers_without_detach() {
        // Valid commands
        assert_ok!(QueuedContainer::new("docker run -d some_image"));
        assert_ok!(QueuedContainer::new("docker run --detach some_image"));
        assert_ok!(QueuedContainer::new("docker run --detach=true some_image"));
        // Invalid commands
        assert_err!(QueuedContainer::new("docker run some_image"));
        assert_err!(QueuedContainer::new("docker run --detach=false some_image"));
    }
}
