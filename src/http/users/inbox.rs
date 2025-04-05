use axum::{extract::Path, response::IntoResponse};

use super::Params;

pub async fn inbox(Path(Params { user_name }): Path<Params>) -> impl IntoResponse {
    "TODO: inbox"
}
