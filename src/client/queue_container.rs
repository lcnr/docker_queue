use super::ClientApp;
use crate::{client::ClientError, domain::QueuedContainer};
use anyhow::{Context, Result};
use tokio::{fs::File, io::AsyncReadExt};

impl<W: std::io::Write> ClientApp<W> {
    pub async fn queue_container(
        &mut self,
        command: String,
        is_path: bool,
        paused: bool,
    ) -> Result<()> {
        let client = reqwest::Client::new();
        let command = if is_path {
            let mut f = File::open(&command).await?;
            let mut buffer = String::new();
            f.read_to_string(&mut buffer).await?;
            buffer
        } else {
            command
        };

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

        Ok(())
    }
}
