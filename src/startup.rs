use axum::{
    body::Body,
    extract::Request,
    routing::{get, post},
    serve::Serve,
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

use crate::routes::{health_check, subscribe};

pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> Result<Serve<Router, Router>, std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = Uuid::new_v4();
                tracing::span!(
                    tracing::Level::INFO,
                    "request",
                    method = tracing::field::display(request.method()),
                    uri = tracing::field::display(request.uri()),
                    version = tracing::field::debug(request.version()),
                    request_id = tracing::field::display(request_id)
                )
            }),
        )
        .with_state(connection);

    let server = axum::serve(listener, app);

    Ok(server)
}
