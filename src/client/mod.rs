mod list_containers;
mod queue_container;
mod start_container;

pub struct ClientApp<W: std::io::Write> {
    pub port: u16,
    pub writer: W,
}

impl<W: std::io::Write> ClientApp<W> {
    pub fn new(port: u16, writer: W) -> Self {
        Self { port, writer }
    }
}
