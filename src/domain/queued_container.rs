use std::path::Path;

use crate::error_chain_fmt;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};
use uuid::Uuid;

#[derive(thiserror::Error)]
pub enum QueuedContainerError {
    #[error("Invalid docker run command: {0}")]
    InvalidQueuedCommand(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
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
            return Err(QueuedContainerError::InvalidQueuedCommand(format!(
                "Should start with \"docker run\": {:?}",
                command
            )));
        }
        let command = command.replace("\n", " ").replace("\\", "");
        let detach_flags = command
            .split_whitespace()
            .skip(2)
            .take_while(|x| x.starts_with('-'))
            .filter(|&x| (x == "-d") | (x == "--detach") | (x == "--detach=true"))
            .count();

        if detach_flags != 1 {
            return Err(QueuedContainerError::InvalidQueuedCommand(format!(
                "Include a detach flag such as: \"-d\" or \"--detach\": {:?}",
                command
            )));
        }

        Ok(Self {
            id,
            command,
            status: QueuedContainerStatus::Paused,
        })
    }

    pub async fn from_path(path: impl AsRef<Path>) -> Result<Self, QueuedContainerError> {
        let mut f = File::open(path).await.context("Failed to open path.")?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)
            .await
            .context("Failed to read file.")?;
        let command = buffer
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n");
        Self::new(command)
    }

    pub fn get_cmd_args(&self) -> Result<Vec<String>> {
        let args = shellwords::split(self.command.split_once(' ').unwrap().1)
            .context("Failed to split args.")?;
        Ok(args)
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
    use test_case::test_case;

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

    #[test]
    fn get_cmd_args_handle_quoted_strings() {
        let command = "docker run -d --rm alpine sh -c \"sleep 30 && echo something\"";
        let container = QueuedContainer::new(command).unwrap();
        let args = container.get_cmd_args().unwrap();
        assert_eq!(
            args,
            vec![
                "run",
                "-d",
                "--rm",
                "alpine",
                "sh",
                "-c",
                "sleep 30 && echo something"
            ]
        );
    }

    #[test_case("docker run --rm -d \n\talpine sleep 3\n"; "with spaces")]
    #[test_case("docker run --rm -d\\\n\talpine sleep 3\n"; "without spaces")]
    #[test_case("docker run\\\n--rm\\\n-d\\\n\talpine sleep 3\n"; "more lines")]
    fn get_cmd_args_handle_multiple_lines<'a>(command: &'a str) {
        let container = QueuedContainer::new(command).unwrap();
        let args = container.get_cmd_args().unwrap();
        assert_eq!(args, vec!["run", "--rm", "-d", "alpine", "sleep", "3"]);
    }

    #[test_case("tests/examples/one_line.sh"; "One line")]
    #[test_case("tests/examples/two_lines.sh"; "Two lines")]
    #[test_case("tests/examples/with_blankline.sh"; "With blank line")]
    #[test_case("tests/examples/with_bash.sh"; "With bash comment")]
    #[tokio::test]
    async fn create_queued_container_from_path<'a>(path: &'a str) {
        let queued_container = QueuedContainer::from_path(path).await;
        assert_ok!(queued_container);
    }
}
