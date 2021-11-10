use super::State;
use crate::{
    domain::{QueuedContainer, RunningContainerId},
    error_chain_fmt,
};
use anyhow::{Context, Result};
use bollard::Docker;
use futures::TryStreamExt;
use std::sync::Arc;
use tokio::{process::Command, sync::mpsc};
use tracing::{debug, error, info, Instrument};

#[derive(thiserror::Error)]
pub enum LauncherTaskError {
    #[error("Error launching \"docker run\": {0}")]
    RunContainerError(String),
    #[error(transparent)]
    WaitContainerError(#[from] bollard::errors::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
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
    /// Indicates the current running container has finished.
    RunningFinished,
    Error(LauncherTaskError),
}

impl From<LauncherTaskError> for TaskMessage {
    fn from(error: LauncherTaskError) -> Self {
        Self::Error(error)
    }
}

#[tracing::instrument(name = "Launcher task", skip(state, tx, rx))]
pub(super) async fn start_launcher_task(
    state: Arc<State>,
    tx: mpsc::Sender<TaskMessage>,
    mut rx: mpsc::Receiver<TaskMessage>,
) {
    while let Some(msg) = rx.recv().await {
        info!("Received: {:?}", msg);
        let result = match msg {
            TaskMessage::CheckRun => match state.run_first_container_in_queue().await {
                Ok(Some(id)) => {
                    let tx = tx.clone();
                    tokio::spawn({
                        async {
                            wait_for_container(id, tx).await;
                        }
                        .instrument(tracing::Span::current())
                    });
                    Ok(())
                }
                Ok(None) => Ok(()),
                Err(error) => Err(error),
            },
            TaskMessage::RunningFinished => {
                *state.running_container.lock().unwrap() = None;
                tx.send(TaskMessage::CheckRun)
                    .await
                    .expect("Receiver dropped.");
                Ok(())
            }
            TaskMessage::Error(error) => Err(error),
        };
        if let Err(error) = result {
            error!("Launcher task error: {:?}", error);
        }
    }
}

impl State {
    #[tracing::instrument(name = "Run first container in queue", skip(self))]
    async fn run_first_container_in_queue(
        &self,
    ) -> Result<Option<RunningContainerId>, LauncherTaskError> {
        if !self.is_running() {
            let container = self.queued_containers.lock().unwrap().pop_front();
            if let Some(container) = container {
                let id = run_container(container).await?;
                *self.running_container.lock().unwrap() = Some(id.clone());
                return Ok(Some(id));
            }
        }
        Ok(None)
    }

    fn is_running(&self) -> bool {
        self.running_container.lock().unwrap().is_some()
    }
}

#[tracing::instrument(name = "Run container", skip(container), fields(container = %container.id()))]
async fn run_container(
    container: QueuedContainer,
) -> Result<RunningContainerId, LauncherTaskError> {
    let output = Command::new("docker")
        .args(container.get_cmd_args()?)
        .output()
        .await
        .context("Failed to execute docker run command.")?;

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).context("Failed to parse stderr.")?;
        return Err(LauncherTaskError::RunContainerError(stderr));
    }

    let stdout = String::from_utf8(output.stdout).context("Failed to parse stdout.")?;
    let id = RunningContainerId::new(stdout);
    info!("Running id: {:?}", id.as_ref());
    Ok(id)
}

#[tracing::instrument(name = "Wait container", skip(tx))]
async fn wait_for_container(id: RunningContainerId, tx: mpsc::Sender<TaskMessage>) {
    match Docker::connect_with_local_defaults() {
        Ok(docker) => {
            match docker
                .wait_container::<&str>(id.as_ref(), None)
                .try_collect::<Vec<_>>()
                .await
            {
                Ok(responses) => {
                    responses.iter().for_each(|response| {
                        debug!("{:?}", response);
                    });
                    tx.send(TaskMessage::RunningFinished)
                        .await
                        .expect("Receiver dropped.");
                }
                Err(error) => {
                    let error = LauncherTaskError::WaitContainerError(error).into();
                    tx.send(error).await.expect("Receiver dropped.");
                }
            }
        }
        Err(error) => {
            let error = LauncherTaskError::UnexpectedError(error.into()).into();
            tx.send(error).await.expect("Receiver dropped.");
        }
    }
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
