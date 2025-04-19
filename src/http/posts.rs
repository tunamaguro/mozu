use axum::{Json, Router, extract::State, routing};

use super::state::AppRegistry;

#[tracing::instrument(skip(registry))]
pub async fn create_post(State(registry): State<AppRegistry>) -> Result<Json<()>, String> {
    todo!()
}

pub fn router(registry: AppRegistry) -> Router {
    Router::new()
        .route("/", routing::post(create_post))
        .with_state(registry)
}
