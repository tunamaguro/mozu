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
        .route("/inbox", routing::post(inbox::inbox))
        .route("/actors/{user_name}", routing::get(actor::actor))
        .route("/actors/{user_name}/inbox", routing::post(inbox::inbox))
        .route("/actors/{user_name}/outbox", routing::post(outbox::outbox))
        .with_state(registry)
}
