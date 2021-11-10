use super::ClientApp;
use crate::domain::RunningContainerId;
use anyhow::{Context, Result};

impl<W: std::io::Write> ClientApp<W> {
    pub async fn get_running_container(&mut self) -> Result<()> {
        let client = reqwest::Client::new();
        let container_id = client
            .get(format!(
                "http://127.0.0.1:{}/get_running_container",
                self.port
            ))
            .send()
            .await
            .context("Failed to execute request.")?
            .json::<Option<RunningContainerId>>()
            .await?;
        let show_id = container_id.as_ref().map(|id| id.as_ref()).unwrap_or("-");
        writeln!(self.writer, "{}", show_id)?;
        Ok(())
    }
}
