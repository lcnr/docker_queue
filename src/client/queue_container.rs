use super::ClientApp;
use crate::{client::ClientError, domain::QueuedContainer};
use anyhow::{Context, Result};

impl<W: std::io::Write> ClientApp<W> {
    pub async fn queue_container(
        &mut self,
        command: impl Into<String>,
        paused: bool,
    ) -> Result<()> {
        let client = reqwest::Client::new();
        let mut queued_container = QueuedContainer::new(command)?;
        if !paused {
            queued_container.queue();
        }

        let response = client
            .post(format!("http://127.0.0.1:{}/queue_container", self.port))
            .json(&queued_container)
            .send()
            .await
            .context("Failed to execute request.")?;

        if !response.status().is_success() {
            return Err(ClientError::ServerStatusError(response.status()).into());
        }

        writeln!(
            self.writer,
            "Container \"{}\" added to queue ({})",
            queued_container.id(),
            queued_container.status()
        )?;

        self.list_containers().await?;

        Ok(())
    }
}
