use super::State;
use crate::{
    domain::{QueuedContainer, RunningContainerId},
    error_chain_fmt,
};
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::{process::Command, sync::mpsc};
use tracing::{error, info};

#[derive(thiserror::Error)]
enum LauncherTaskError {
    #[error("Error launching \"docker run\": {0}")]
    RunContainerError(String),
}

impl std::fmt::Debug for LauncherTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Debug)]
pub enum TaskMessage {
    /// Check if there is any queued container ready and run it if possible.
    CheckRun,
}

#[tracing::instrument(name = "Launcher task", skip(state, rx))]
pub(super) async fn start_launcher_task(state: Arc<State>, mut rx: mpsc::Receiver<TaskMessage>) {
    while let Some(msg) = rx.recv().await {
        info!("Received: {:?}", msg);
        match msg {
            TaskMessage::CheckRun => {
                if let Err(error) = state.run_first_container_in_queue().await {
                    error!("Error running first container in queue: {:?}", error);
                }
            }
        }
    }
}

impl State {
    #[tracing::instrument(name = "Run first container in queue", skip(self))]
    async fn run_first_container_in_queue(&self) -> Result<()> {
        let container = self.queued_containers.lock().unwrap().pop_front();
        if let Some(container) = container {
            let _id = run_container(container).await?;
        }
        Ok(())
    }

    // async fn is_queue_ready(&self) -> Result<bool> {
    //     let n = self
    //         .get_containers()
    //         .await?
    //         .into_iter()
    //         .filter(|container| match container {
    //             Container::Running(_) => true,
    //             Container::Queued(_) => false, // TODO: check time order
    //         })
    //         .count();
    //     Ok(n == 0)
    // }
}

// #[tracing::instrument(name = "Run container", skip(container), fields(container = %container.id()))]
async fn run_container(container: QueuedContainer) -> Result<RunningContainerId> {
    info!("{:#?}", container.get_cmd_args());
    let output = Command::new("docker")
        .args(container.get_cmd_args()?)
        .output()
        .await
        .context("Failed to execute docker run command.")?;

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).context("Failed to parse stderr.")?;
        return Err(LauncherTaskError::RunContainerError(stderr).into());
    }

    let stdout = String::from_utf8(output.stdout).context("Failed to parse stdout.")?;
    let id = RunningContainerId::new(stdout);
    info!("Running id: {:?}", id.as_ref());
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::get_running_containers;

    #[tokio::test]
    async fn run_container_works() {
        let container = QueuedContainer::new("docker run --rm -d alpine sleep 5").unwrap();
        let id = run_container(container).await.unwrap();
        println!("{:#?}", id.as_ref());
        let running_containers = get_running_containers()
            .await
            .unwrap()
            .into_iter()
            .filter_map(|container| container.id)
            .filter(|running_id| running_id == id.as_ref())
            .count();
        assert_eq!(1, running_containers);
    }
}
