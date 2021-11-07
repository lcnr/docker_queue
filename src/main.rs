use anyhow::Result;
use clap::Parser;
use docker_queue::{
    client::ClientApp,
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
    /// List containers
    List,
    /// Start server
    Serve,
    /// Queue container
    Queue,
    /// Remove container
    Remove,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    if let SubCommand::Serve = opts.subcmd {
        let subscriber = get_subscriber("docker_queue".into(), "info".into(), std::io::stdout);
        init_subscriber(subscriber);
        let app = Server::build(Settings { port: opts.port })?;
        app.start().await?;
    } else {
        let mut client = ClientApp::new(opts.port, std::io::stdout());
        match opts.subcmd {
            SubCommand::List => {
                client.list_containers().await?;
            }
            SubCommand::Queue => todo!(),
            SubCommand::Remove => todo!(),
            SubCommand::Serve => {}
        }
    }

    Ok(())
}
