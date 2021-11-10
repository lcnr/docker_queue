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

const COMMAND_MAX_LEN: usize = 40;

struct ShowContainerBuilder {
    show_all: bool,
    status: String,
    id: String,
    image: String,
    command: String,
    created: String,
    names: String,
}

impl ShowContainerBuilder {
    fn build(self) -> ShowContainer {
        let mut command = self.command;
        if !self.show_all && (command.len() > COMMAND_MAX_LEN) {
            command = command
                .chars()
                .take(COMMAND_MAX_LEN - 3)
                .chain("...".chars())
                .collect();
        }
        ShowContainer {
            status: self.status,
            id: self.id,
            image: self.image,
            command,
            created: self.created,
            names: self.names,
        }
    }

    fn show_all(mut self, show_all: bool) -> Self {
        self.show_all = show_all;
        self
    }
}

impl Default for ShowContainerBuilder {
    fn default() -> Self {
        Self {
            status: "-".to_string(),
            id: "-".to_string(),
            image: "-".to_string(),
            command: "-".to_string(),
            created: "-".to_string(),
            names: "-".to_string(),
            show_all: false,
        }
    }
}

impl From<Container> for ShowContainerBuilder {
    fn from(container: Container) -> Self {
        let builder = match container {
            Container::Running(container) => ShowContainerBuilder {
                status: "Running".to_string(),
                id: container.id.unwrap_or_else(|| "-".to_string()),
                image: container.image.unwrap_or_else(|| "-".to_string()),
                command: container.command.unwrap_or_else(|| "-".to_string()),
                created: container
                    .created
                    .map(|o| o.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                names: container
                    .names
                    .map(|o| o.concat())
                    .unwrap_or_else(|| "-".to_string()),
                ..Default::default()
            },
            Container::Queued(container) => ShowContainerBuilder {
                status: container.status().to_string(),
                id: container.id(),
                command: container.command().to_string(),
                ..Default::default()
            },
        };
        builder
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
        // lens[5] = lens[5].max(container.names.len());
    });
    lens[5] = 0;
    lens.iter_mut().for_each(|len| *len += pad);
    lens
}

fn get_print_line(container: ShowContainer, max_lens: [usize; 6]) -> String {
    let line = [
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
    .collect::<String>();
    if line.starts_with("Running") {
        return style(line).bold().green().to_string();
    } else if line.starts_with("Paused") {
        return style(line).color256(8).to_string();
    }
    line
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

    pub async fn list_containers(&mut self, show_all: bool) -> Result<()> {
        let containers = self
            .get_containers()
            .await?
            .into_iter()
            .map(|container| {
                ShowContainerBuilder::from(container)
                    .show_all(show_all)
                    .build()
            })
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
