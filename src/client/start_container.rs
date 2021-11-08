use crate::domain::QueuedContainer;

use super::ClientApp;
use anyhow::Result;

impl<W: std::io::Write> ClientApp<W> {
    pub async fn start_container(&mut self, queued_container: QueuedContainer) -> Result<()> {
        writeln!(
            self.writer,
            "Running \"{}\": \"{}\"",
            queued_container.id(),
            queued_container.command()
        )?;

        Ok(())
    }
}
