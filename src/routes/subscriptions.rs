use axum::{
    extract::State,
    http::StatusCode,
    response::{Form, IntoResponse},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    State(connection): State<PgPool>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving subscriber details in database",);

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&connection)
    .instrument(query_span)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            tracing::error!("Failed to execute query: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
