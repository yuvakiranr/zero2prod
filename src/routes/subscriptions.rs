use axum::{
    extract::State,
    http::StatusCode,
    response::{Form, IntoResponse},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
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
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            println!("Failed to execute query: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
