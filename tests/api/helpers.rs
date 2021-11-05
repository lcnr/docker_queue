use docker_queue::{configuration::Settings, server::Server, telemetry::{get_subscriber, init_subscriber}};
use once_cell::sync::Lazy;

// Ensure that 'tracing' stack is only initialized once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    // Set up tracing
    Lazy::force(&TRACING);

    let app = Server::build(Settings { port: 0 }).expect("Failed to build application.");
    let port = app.port();
    let _ = tokio::spawn(async move { app.start().await });

    TestApp { port }
}
