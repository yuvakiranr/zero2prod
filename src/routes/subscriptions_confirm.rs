use axum::extract::{Query, State};
use axum::response::IntoResponse;

use reqwest::StatusCode;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(params))]
pub async fn confirm(Query(params): Query<Parameters>) -> impl IntoResponse {
    StatusCode::OK
}
