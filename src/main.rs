use axum::{routing::post, Json, Router};
use kube::core::conversion::{ConversionRequest, ConversionResponse, ConversionReview};
use stackable_webhook::{Options, WebhookServer};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", post(convert));

    let options = Options::builder()
        .disable_redirect()
        .socket_addr(([0, 0, 0, 0], 8443))
        .build();
    let server = WebhookServer::new(router, options);
    server.run().await
}

async fn convert(Json(review): Json<ConversionReview>) -> Json<ConversionReview> {
    println!("{}", serde_json::to_string(&review).unwrap());
    let request = ConversionRequest::from_review(review).unwrap();
    let objects = request.objects.clone();
    let response: ConversionResponse = ConversionResponse::for_request(request).success(objects);
    println!("{}", serde_json::to_string(&response).unwrap());
    Json(response.into_review())
}
