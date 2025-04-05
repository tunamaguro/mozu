use axum::{extract::Path, response::IntoResponse};

use super::Params;

#[tracing::instrument(skip_all)]
pub async fn outbox(Path(_): Path<Params>) -> impl IntoResponse {
    "TODO: outbox"
}
