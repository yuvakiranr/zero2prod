use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> Result<Serve<Router, Router>, std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(connection);

    let server = axum::serve(listener, app);

    Ok(server)
}
