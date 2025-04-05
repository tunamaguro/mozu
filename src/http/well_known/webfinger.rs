use std::borrow::Cow;

use axum::{
    Json,
    extract::{Query, State},
    http::{self, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;

use crate::{
    ap::{WebFinger, WebFingerLink, constants, webfinger::AcctUri},
    domain::account::model::{AccountName, FindAccountError},
    http::state::{AppRegistry, AppRegistryExt},
};

pub struct ApiSuccess<T: Serialize> {
    status: StatusCode,
    data: T,
}

impl<T: Serialize> ApiSuccess<T> {
    pub fn new(status: StatusCode, data: T) -> Self {
        Self { status, data }
    }
}
impl<T: Serialize> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            [(http::header::CONTENT_TYPE, constants::WEBFINGER_MEDIA_TYPE)],
            Json(self.data),
        )
            .into_response()
    }
}

pub enum ApiError {
    InternalServerError,
    BadRequest(Cow<'static, str>),
    NotFound(Cow<'static, str>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND, message).into_response(),
        }
    }
}

impl From<FindAccountError> for ApiError {
    fn from(err: FindAccountError) -> Self {
        match err {
            FindAccountError::InvalidName(_) => ApiError::BadRequest("invalid name".into()),
            FindAccountError::Unknown(_) => ApiError::InternalServerError,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct WebFingerQuery {
    resource: AcctUri,
}

impl WebFingerQuery {
    fn try_into_domain(&self, host: &str) -> Result<AccountName, ApiError> {
        if self.resource.host != host {
            return Err(ApiError::BadRequest(r#"other host"#.into()));
        }
        let account_name = AccountName::new(&self.resource.user)
            .map_err(|e| ApiError::BadRequest(e.to_string().into()))?;

        Ok(account_name)
    }
}

#[axum::debug_handler]
#[tracing::instrument(skip_all)]
pub async fn webfinger(
    State(registry): State<AppRegistry>,
    Query(query): Query<WebFingerQuery>,
) -> Result<ApiSuccess<WebFinger>, ApiError> {
    let host_service = registry.host_url_service();
    let host = host_service.host();
    let account_name = query.try_into_domain(host)?;

    let account_service = registry.account_service();
    let account = account_service.find_by_name(&account_name).await?;

    let Some(account) = account else {
        return Err(ApiError::NotFound(
            format!("account {} not found", account_name.as_str()).into(),
        ));
    };

    let links = vec![
        WebFingerLink::builder()
            .rel("self")
            .kind("application/activity+json")
            .href(host_service.user_url(account.name().as_str()))
            .build(),
    ];

    let webfinger = WebFinger::builder()
        .subject(format!("acct:{}@{}", account.name().as_str(), host))
        .links(links)
        .build();

    Ok(ApiSuccess::new(StatusCode::OK, webfinger))
}
