use super::ClientApp;
use crate::domain::RunningContainerId;
use anyhow::{Context, Result};

impl<W: std::io::Write> ClientApp<W> {
    pub async fn get_running_container(&mut self) -> Result<()> {
        let client = reqwest::Client::new();
        let id = client
            .get(format!(
                "http://127.0.0.1:{}/get_running_container",
                self.port
            ))
            .send()
            .await
            .context("Failed to execute request.")?
            .json::<RunningContainerId>()
            .await?;
        writeln!(self.writer, "{}", id.as_ref())?;
        Ok(())
    }
}
