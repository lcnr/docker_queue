use super::ClientApp;
use crate::domain::Container;
use anyhow::{Context, Result};
use console::{pad_str, style, Alignment};

struct ShowContainer {
    status: String,
    id: String,
    image: String,
    command: String,
    created: String,
    names: String,
}

impl From<Container> for ShowContainer {
    fn from(container: Container) -> Self {
        match container {
            // Container::Ignored(_) => todo!(),
            Container::Running(container) => ShowContainer {
                status: "Running".into(),
                id: container.id.unwrap_or_else(|| "-".into()),
                image: container.image.unwrap_or_else(|| "-".into()),
                command: container.command.unwrap_or_else(|| "-".into()),
                created: container
                    .created
                    .map(|o| o.to_string())
                    .unwrap_or_else(|| "-".into()),
                names: container
                    .names
                    .map(|o| o.concat())
                    .unwrap_or_else(|| "-".into()),
            },
            Container::Queued(container) => ShowContainer {
                status: container.status().to_string(),
                id: container.id(),
                image: "-".into(),
                command: container.command().into(),
                created: "-".into(),
                names: "-".into(),
            },
        }
    }
}

fn get_max_lens(containers: &[ShowContainer], pad: usize) -> [usize; 6] {
    let mut lens = HEADERS.map(|o| o.len());
    containers.iter().for_each(|container| {
        lens[0] = lens[0].max(container.status.len());
        lens[1] = lens[1].max(container.id.len());
        lens[2] = lens[2].max(container.image.len());
        lens[3] = lens[3].max(container.command.len());
        lens[4] = lens[4].max(container.created.len());
        lens[5] = lens[5].max(container.names.len());
    });
    lens.iter_mut().for_each(|len| *len += pad);
    lens
}

fn get_print_line(container: ShowContainer, max_lens: [usize; 6]) -> String {
    [
        container.status,
        container.id,
        container.image,
        container.command,
        container.created,
        container.names,
    ]
    .iter()
    .zip(max_lens)
    .map(|(s, width)| pad_str(s, width, Alignment::Left, None))
    .collect()
}

const HEADERS: [&str; 6] = ["status", "id", "image", "command", "created", "names"];

impl<W: std::io::Write> ClientApp<W> {
    pub async fn get_containers(&self) -> Result<Vec<Container>> {
        let client = reqwest::Client::new();
        client
            .get(format!("http://127.0.0.1:{}/list_containers", self.port))
            .send()
            .await
            .context("Failed to execute request.")?
            .json::<Vec<Container>>()
            .await
            .context("Failed to deserealize containers.")
    }

    pub async fn list_containers(&mut self) -> Result<()> {
        let containers = self
            .get_containers()
            .await?
            .into_iter()
            .map(ShowContainer::from)
            .collect::<Vec<_>>();

        let max_lens = get_max_lens(&containers, 2);
        let headers = HEADERS
            .iter()
            .zip(max_lens)
            .map(|(header, len)| pad_str(header, len, Alignment::Left, None))
            .collect::<String>();
        writeln!(self.writer, "{}", style(headers).bold())?;

        for container in containers {
            writeln!(self.writer, "{}", get_print_line(container, max_lens))?;
        }

        Ok(())
    }
}
