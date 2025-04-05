mod inbox;
mod outbox;
use crate::{
    ap::{Actor, ActorType, Context},
    domain::account::model::{AccountName, AccountNameError, FindAccountError},
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing,
};
use serde::{Deserialize, Serialize};

use super::{
    state::{AppRegistry, AppRegistryExt},
    utils::ActivityJson,
};

#[derive(Serialize)]
pub enum ApiError {
    NotFound,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::NotFound => (axum::http::StatusCode::NOT_FOUND, "Not found").into_response(),
            ApiError::InternalServerError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response(),
        }
    }
}

impl From<AccountNameError> for ApiError {
    fn from(err: AccountNameError) -> Self {
        match err {
            AccountNameError::InvalidName(_) => ApiError::NotFound,
        }
    }
}

impl From<FindAccountError> for ApiError {
    fn from(err: FindAccountError) -> Self {
        match err {
            FindAccountError::InvalidName(_) => ApiError::NotFound,
            FindAccountError::Unknown(_) => ApiError::InternalServerError,
        }
    }
}

#[derive(Deserialize)]
struct Params {
    pub(crate) user_name: String,
}

async fn user(
    State(registry): State<AppRegistry>,
    Path(params): Path<Params>,
) -> Result<impl IntoResponse, ApiError> {
    let account_name = AccountName::new(&params.user_name)?;
    let account_service = registry.account_service();
    let account = account_service.find_by_name(&account_name).await?;
    let Some(_) = account else {
        return Err(ApiError::NotFound);
    };

    let hosturl_service = registry.host_url_service();
    let actor = Actor::builder()
        .kind(ActorType::Person)
        .id(hosturl_service.user_url(account_name.as_str()))
        .inbox(hosturl_service.inbox_url(account_name.as_str()))
        .outbox(hosturl_service.outbox_url(account_name.as_str()))
        .preferred_username(account_name.as_str())
        .build();

    Ok(ActivityJson(Context::new(actor)))
}

pub fn router(registry: AppRegistry) -> axum::Router {
    axum::Router::new()
        .route("/{user_name}", routing::get(user))
        .route("/{user_name}/inbox", routing::get(inbox::inbox))
        .route("/{user_name}/outbox", routing::get(outbox::outbox))
        .with_state(registry)
}
