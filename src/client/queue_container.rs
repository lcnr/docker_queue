use crate::domain::QueuedContainer;
use anyhow::{Context, Result};

pub async fn queue_container(name: &str, port: u16, w: &mut impl std::io::Write) -> Result<()> {
    let client = reqwest::Client::new();
    let queued_container = QueuedContainer::new(name);

    let response = client
        .post(format!("http://127.0.0.1:{}/queue_container", port))
        .json(&queued_container)
        .send()
        .await
        .context("Failed to execute request.")?;

    if !response.status().is_success() {
        writeln!(w, "status error")?;
    }

    writeln!(w, "{} added to queue", name)?;

    Ok(())
}
