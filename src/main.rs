use std::error::Error;

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use kube::core::conversion::{ConversionRequest, ConversionResponse, ConversionReview};
use stackable_telemetry::Tracing;
use stackable_webhook::{Options, WebhookServer};
use tracing::level_filters::LevelFilter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Ping {
    pub ping: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Pong {
    pub pong: String,
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _trace_guard = Tracing::builder()
        .service_name("dummy-webhook")
        .with_console_output("CONSOLE_LOG_LEVEL", LevelFilter::INFO)
        .with_otlp_trace_exporter("OTLP_TRACE_LEVEL", LevelFilter::TRACE)
        // .with_otlp_log_exporter("OTLP_LOG_LEVEL", LevelFilter::DEBUG)
        .build()
        .init()?;

    let router = Router::new()
        .route("/", post(convert))
        .route("/", get(index))
        .route("/ping/:name", post(ping));

    let options = Options::builder().bind_address([0, 0, 0, 0], 8443).build();
    let server = WebhookServer::new(router, options);

    server.run().await?;
    tracing::info!("shutting down");

    Ok(())
}

#[tracing::instrument(target = "convert", skip(review))]
async fn convert(Json(review): Json<ConversionReview>) -> Json<ConversionReview> {
    println!("{}", serde_json::to_string(&review).unwrap());
    let request = ConversionRequest::from_review(review).unwrap();
    let objects = request.objects.clone();
    let response: ConversionResponse = ConversionResponse::for_request(request).success(objects);
    println!("{}", serde_json::to_string(&response).unwrap());
    Json(response.into_review())
}

#[tracing::instrument(target = "ping")]
async fn ping(Path(name): Path<String>, Json(ping): Json<Ping>) -> Json<Pong> {
    let pong = Pong {
        pong: ping.ping,
        name,
    };

    tracing::info!("ping-pong");
    Json(pong)
}

#[tracing::instrument(name = "index")]
async fn index() -> impl IntoResponse {
    "Hello"
}
