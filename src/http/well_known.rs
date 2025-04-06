pub(crate) mod webfinger;

use axum::{Router, routing};

use super::state::AppRegistry;

pub fn router(app_registry: AppRegistry) -> Router {
    Router::new()
        .route("/webfinger", routing::get(webfinger::webfinger))
        .with_state(app_registry)
}
