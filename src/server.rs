use crate::configuration::Settings;
use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use std::net::TcpListener;
use tower_http::{
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info_span, Level};
use tracing_futures::WithSubscriber;

pub struct Server {
    listener: TcpListener,
    port: u16,
    app: Router,
}

impl Server {
    pub fn build(configuration: Settings) -> Result<Self> {
        tracing::info!("Configuration: {:?}", configuration);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.port))?;
        let port = listener.local_addr()?.port();
        tracing::info!("Serving at: http://127.0.0.1:{}", port);
        let app = Router::new()
            .route("/health_check", get(health_check))
            .route("/list_containers", get(list_containers))
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
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn start(self) -> Result<()> {
        axum::Server::from_tcp(self.listener)?
            .serve(self.app.into_make_service())
            .await?;
        Ok(())
    }
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument(
    name = "List containers",
    // skip(expected_password_hash, password_candidate)
)]
async fn list_containers() -> StatusCode {
    StatusCode::OK
}
