use super::State;
use crate::{
    configuration::Settings,
    server::{get_running_container, list_containers, queue_container, start_launcher_task},
};
use anyhow::Result;
use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};
use std::{net::TcpListener, sync::Arc};
use tokio::{sync::mpsc, task::JoinHandle};
use tower_http::{
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{
    log::{error, info},
    Level,
};

pub struct Server {
    listener: TcpListener,
    port: u16,
    app: Router,
    launcher_task: JoinHandle<()>,
}

pub async fn health_check() {}

impl Server {
    pub fn build(configuration: Settings) -> Result<Self> {
        tracing::info!("Configuration: {:?}", configuration);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.port))?;
        let port = listener.local_addr()?.port();
        let shared_state = Arc::new(State::new());
        let (tx, rx) = mpsc::channel(8);
        let launcher_task = tokio::spawn({
            let shared_state = Arc::clone(&shared_state);
            let tx = tx.clone();
            async move { start_launcher_task(shared_state, tx, rx).await }
        });

        let app = Router::new()
            .route("/health_check", get(health_check))
            .route("/list_containers", get(list_containers))
            .route("/queue_container", post(queue_container))
            .route("/get_running_container", get(get_running_container))
            .layer(AddExtensionLayer::new(shared_state))
            .layer(AddExtensionLayer::new(tx))
            .layer(
                // More on TraceLayer: https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html
                TraceLayer::new_for_http()
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            );

        Ok(Self {
            listener,
            port,
            app,
            launcher_task,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn start(self) -> Result<()> {
        tracing::info!("Serving at: http://127.0.0.1:{}", self.port);
        let server_task =
            axum::Server::from_tcp(self.listener)?.serve(self.app.into_make_service());

        tokio::select! {
            _ = server_task => {
                info!("Server task terminated.");
            }
            res = self.launcher_task => {
                if let Err(error) = res {
                    error!("{:?}", error);
                }
                info!("Launcher task terminated.");
            }
        }

        Ok(())
    }
}
