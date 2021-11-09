use super::State;
use crate::domain::Container;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

#[derive(Debug)]
pub enum TaskMessage {
    CheckRun,
}

impl State {
    async fn is_queue_ready(&self) -> Result<bool> {
        let n = self
            .get_containers()
            .await?
            .into_iter()
            .filter(|container| match container {
                Container::Running(_) => true,
                Container::Queued(_) => false, // TODO: check time order
            })
            .count();
        Ok(n == 0)
    }
}

#[tracing::instrument(name = "Launcher task", skip(state, rx))]
pub(super) async fn start_launcher_task(state: Arc<State>, mut rx: mpsc::Receiver<TaskMessage>) {
    while let Some(msg) = rx.recv().await {
        info!("Received: {:?}", msg);
        match msg {
            TaskMessage::CheckRun => todo!(),
        }
    }
}
