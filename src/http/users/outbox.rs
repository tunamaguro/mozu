use axum::{extract::Path, response::IntoResponse};

use super::Params;

pub async fn outbox(Path(Params { user_name }): Path<Params>) -> impl IntoResponse {
    "TODO: outbox"
}
