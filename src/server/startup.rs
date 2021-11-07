use crate::{
    configuration::Settings,
    error_chain_fmt,
    server::{health_check, list_containers, queue_container},
};
use anyhow::Result;
use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::{convert::Infallible, net::TcpListener};
use tower_http::{
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

#[derive(thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl IntoResponse for ServerError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_message) = match self {
            ServerError::UnexpectedError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

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
            .route("/queue_container", post(queue_container))
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
