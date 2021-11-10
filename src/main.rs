use anyhow::Result;
use clap::Parser;
use docker_queue::{
    client::ClientApp,
    configuration::Settings,
    server::Server,
    telemetry::{get_subscriber, init_subscriber},
};
use tracing::debug;

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
    Queue(QueueContainer),
    /// Remove container
    Remove,
}

#[derive(Debug, Parser)]
struct QueueContainer {
    /// A docker run command, should include a detach flag as "-d" or "--detach"
    command: String,
    /// Treats the command as a file path to read.
    #[clap(long)]
    path: bool,
    /// The container gets queued but not started even if the queue is empty
    #[clap(long)]
    paused: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let subscriber = get_subscriber("docker_queue".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    debug!("{:#?}", opts);

    if let SubCommand::Serve = opts.subcmd {
        let app = Server::build(Settings { port: opts.port })?;
        app.start().await?;
    } else {
        let mut client = ClientApp::new(opts.port, std::io::stdout());
        match opts.subcmd {
            SubCommand::List => client.list_containers().await?,
            SubCommand::Queue(opts) => client.queue_container(opts.command, opts.path, opts.paused).await?,
            SubCommand::Remove => todo!(),
            SubCommand::Serve => {}
        }
    }

    Ok(())
}
