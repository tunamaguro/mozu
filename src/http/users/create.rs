use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{
    domain::account::model::{
        AccountName, AccountNameError, CreateAccountError, CreateAccountRequest,
    },
    http::state::{AppRegistry, AppRegistryExt},
};

#[derive(Debug, Deserialize)]
pub struct CreateAccountJson {
    username: String,
}

#[derive(Debug, Serialize)]
pub struct CreateAccountResponseJson {
    id: String,
    username: String,
}

pub enum ApiError {
    BadRequest(String),
    Conflict,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            ApiError::Conflict => (StatusCode::CONFLICT, "Account already exists").into_response(),
            ApiError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

impl From<AccountNameError> for ApiError {
    fn from(err: AccountNameError) -> Self {
        match err {
            AccountNameError::InvalidName(name) => {
                ApiError::BadRequest(format!("Invalid name: {}", name))
            }
        }
    }
}

impl From<CreateAccountError> for ApiError {
    fn from(err: CreateAccountError) -> Self {
        match err {
            CreateAccountError::AlreadyExists => ApiError::Conflict,
            CreateAccountError::Unknown(_) => ApiError::InternalServerError,
        }
    }
}

#[tracing::instrument(skip_all)]
pub async fn create(
    State(registry): State<AppRegistry>,
    Json(payload): Json<CreateAccountJson>,
) -> Result<impl IntoResponse, ApiError> {
    let account_name = AccountName::new(&payload.username)?;
    let req = CreateAccountRequest::new(account_name);

    let account_service = registry.account_service();
    let account = account_service.create(req).await?;

    let response = CreateAccountResponseJson {
        id: account.id().to_string(),
        username: account.name().as_str().to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}
