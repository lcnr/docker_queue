use anyhow::Result;
use docker_queue::{
    configuration::Settings,
    server::Server,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("docker_queue".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let app = Server::build(Settings { port: 12000 })?;
    app.start().await?;

    Ok(())
}
