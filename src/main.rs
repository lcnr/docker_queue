use anyhow::Result;
// use bollard::{container::ListContainersOptions, Docker};
use docker_queue::{
    configuration::Settings,
    server::Server,
    telemetry::{get_subscriber, init_subscriber},
};
// use std::collections::HashMap;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let app = Server::build(Settings { port: 12000 })?;
    app.start().await?;

    // let docker = Docker::connect_with_local_defaults()?;

    // let filters = HashMap::from([("status", vec!["running"])]);
    // let options = Some(ListContainersOptions {
    //     all: true,
    //     filters,
    //     ..Default::default()
    // });
    // let containers = docker.list_containers(options).await?;

    // for container in containers {
    //     println!("-> {:#?}", container);
    // }

    Ok(())
}
