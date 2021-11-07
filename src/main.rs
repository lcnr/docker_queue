use anyhow::Result;
use clap::Parser;
use docker_queue::{
    configuration::Settings,
    server::Server,
    telemetry::{get_subscriber, init_subscriber},
};

#[derive(Debug, Parser)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    Serve,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);

    let subscriber = get_subscriber("docker_queue".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let app = Server::build(Settings { port: 12000 })?;
    app.start().await?;

    Ok(())
}
