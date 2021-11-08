use super::ClientApp;
use crate::domain::QueuedContainer;
use anyhow::{Context, Result};

impl<W: std::io::Write> ClientApp<W> {
    pub async fn is_queue_ready(&self, _container: QueuedContainer) -> Result<bool> {
        let n = self
            .get_containers()
            .await?
            .into_iter()
            .filter(|container| match container {
                crate::domain::Container::Running(_) => true,
                crate::domain::Container::Queued(_) => false, // TODO: check time order
            })
            .count();
        Ok(n == 0)
    }

    pub async fn run_container(&self, _container: QueuedContainer) -> Result<()> {
        Ok(())
    }

    pub async fn queue_container(
        &mut self,
        command: impl Into<String>,
        paused: bool,
    ) -> Result<()> {
        let client = reqwest::Client::new();
        let queued_container = QueuedContainer::new(command, paused)?;

        let response = client
            .post(format!("http://127.0.0.1:{}/queue_container", self.port))
            .json(&queued_container)
            .send()
            .await
            .context("Failed to execute request.")?;

        if !response.status().is_success() {
            writeln!(self.writer, "status error")?;
        }

        writeln!(
            self.writer,
            "Container \"{}\" added to queue ({})",
            queued_container.id(),
            queued_container.status()
        )?;

        if !paused {
            if self.is_queue_ready(queued_container).await? {
                // if yes run container
            }
        }

        Ok(())
    }
}
