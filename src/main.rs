use axum::{routing::post, Json, Router};
use kube::core::conversion::{ConversionRequest, ConversionResponse};
use stackable_webhook::{Options, WebhookServer};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", post(convert));

    let options = Options::builder().disable_redirect().build();
    let server = WebhookServer::new(router, options);
    server.run().await
}

async fn convert(Json(request): Json<ConversionRequest>) -> Json<ConversionResponse> {
    Json(request.into())
}
