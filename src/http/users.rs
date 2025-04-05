use axum::routing;
use serde::Deserialize;

use super::state::AppRegistry;

mod actor;
mod inbox;
mod outbox;

#[derive(Deserialize)]
pub struct Params {
    pub(crate) user_name: String,
}

pub fn router(registry: AppRegistry) -> axum::Router {
    axum::Router::new()
        .route("/{user_name}", routing::get(actor::actor))
        .route("/{user_name}/inbox", routing::get(inbox::inbox))
        .route("/{user_name}/outbox", routing::get(outbox::outbox))
        .with_state(registry)
}
