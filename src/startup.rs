use axum::{
    body::Body,
    extract::{FromRef, Request},
    routing::{get, post},
    serve::Serve,
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

use crate::{
    configuration::{DatabaseSettings, Settings},
    email_client::EmailClient,
    routes::{confirm, health_check, publish_newsletter, subscribe},
};

pub struct Application {
    port: u16,
    server: Serve<Router, Router>,
}

#[derive(Clone)]
pub struct ApplicationBaseUrl(pub String);

#[derive(Clone)]
pub struct ApplicationState {
    pub db_connection: Pool<Postgres>,
    pub email_client: EmailClient,
    pub base_url: ApplicationBaseUrl,
}

impl FromRef<ApplicationState> for ApplicationBaseUrl {
    fn from_ref(input: &ApplicationState) -> Self {
        input.base_url.clone()
    }
}

impl FromRef<ApplicationState> for Pool<Postgres> {
    fn from_ref(input: &ApplicationState) -> Self {
        input.db_connection.clone()
    }
}

impl FromRef<ApplicationState> for EmailClient {
    fn from_ref(input: &ApplicationState) -> Self {
        input.email_client.clone()
    }
}

pub fn get_connection_pool(confguration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(confguration.with_db())
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_untill_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(
    listener: TcpListener,
    connection: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> Result<Serve<Router, Router>, std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .route("/subscriptions/confirm", get(confirm))
        .route("/newsletters", post(publish_newsletter))
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
        .with_state(ApplicationState {
            db_connection: connection,
            email_client,
            base_url: ApplicationBaseUrl(base_url),
        });

    let server = axum::serve(listener, app);

    Ok(server)
}
