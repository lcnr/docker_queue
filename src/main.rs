use anyhow::Result;
use clap::Parser;
use docker_queue::{
    client::list_containers,
    configuration::Settings,
    server::Server,
    telemetry::{get_subscriber, init_subscriber},
};

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short, long, default_value = "12000")]
    port: u16,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    List,
    Serve,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Serve => {
            let subscriber = get_subscriber("docker_queue".into(), "info".into(), std::io::stdout);
            init_subscriber(subscriber);
            let app = Server::build(Settings { port: opts.port })?;
            app.start().await?;
        }
        SubCommand::List => {
            // TODO healthcheck
            let mut w = std::io::stdout();
            list_containers(opts.port, &mut w).await.unwrap();
        }
    }

    Ok(())
}
