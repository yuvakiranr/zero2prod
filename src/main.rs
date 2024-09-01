use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info,tower_http=debug,axum::rejection=trace".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read config.");
    let connection = PgPoolOptions::new()
        .max_connections(10)
        .connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    run(listener, connection)?.await
}
