use super::ClientApp;
use crate::domain::QueuedContainer;
use anyhow::{Context, Result};

impl<W: std::io::Write> ClientApp<W> {
    pub async fn queue_container(&mut self, name: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let queued_container = QueuedContainer::new(name);

        let response = client
            .post(format!("http://127.0.0.1:{}/queue_container", self.port))
            .json(&queued_container)
            .send()
            .await
            .context("Failed to execute request.")?;

        if !response.status().is_success() {
            writeln!(self.writer, "status error")?;
        }

        writeln!(self.writer, "{} added to queue", name)?;

        // is queue empty?
        // if yes run container

        Ok(())
    }
}
