use crate::domain::Container;
use anyhow::{Context, Result};
use console::{pad_str, style, Alignment};

struct ShowContainer {
    id: String,
    image: String,
    command: String,
    created: String,
    names: String,
}

impl From<Container> for ShowContainer {
    fn from(container: Container) -> Self {
        match container {
            Container::Ignored(_) => todo!(),
            Container::Running(container) => ShowContainer {
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
            Container::Queued(_) => todo!(),
        }
    }
}

fn get_max_lens(containers: &[ShowContainer], pad: usize) -> [usize; 5] {
    let mut lens = [0; 5];
    containers.iter().for_each(|container| {
        lens[0] = lens[0].max(container.id.len());
        lens[1] = lens[1].max(container.image.len());
        lens[2] = lens[2].max(container.command.len());
        lens[3] = lens[3].max(container.created.len());
        lens[4] = lens[4].max(container.names.len());
    });
    lens.iter_mut().for_each(|len| *len += pad);
    lens
}

fn get_print_line(container: ShowContainer, max_lens: [usize; 5]) -> String {
    [
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

const HEADERS: [&str; 5] = ["id", "image", "command", "created", "names"];

pub async fn list_containers(port: u16, w: &mut impl std::io::Write) -> Result<()> {
    let client = reqwest::Client::new();

    let containers = client
        .get(format!("http://127.0.0.1:{}/list_containers", port))
        .send()
        .await
        .context("Failed to execute request.")?
        .json::<Vec<Container>>()
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
    writeln!(w, "{}", style(headers).bold())?;

    for container in containers {
        writeln!(w, "{}", get_print_line(container, max_lens))?;
    }

    Ok(())
}
