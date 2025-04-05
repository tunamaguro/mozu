use crate::ap::constants;
use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};

/// A wrapper for JSON responses that sets the correct content type for ActivityPub
pub struct ActivityJson<T>(pub T);

impl<T> IntoResponse for ActivityJson<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (
            [(header::CONTENT_TYPE, constants::ACTIVITYPUB_MEDIA_TYPE)],
            Json(self.0),
        )
            .into_response()
    }
}

fn is_activity_json_type(headers: &HeaderMap) -> bool {
    let Some(content_type) = headers.get(header::CONTENT_TYPE) else {
        return false;
    };
    let Ok(content_type) = content_type.to_str() else {
        return false;
    };

    let Ok(mime) = content_type.parse::<mime::Mime>() else {
        return false;
    };

    mime == *constants::ACTIVITYPUB_MIME || mime == *constants::ACTIVITYPUB_MIME_ALT
}

pub enum ActivityJsonRejection {
    InvalidContentType,
    Json(JsonRejection),
}

impl From<JsonRejection> for ActivityJsonRejection {
    fn from(rejection: JsonRejection) -> Self {
        ActivityJsonRejection::Json(rejection)
    }
}

impl IntoResponse for ActivityJsonRejection {
    fn into_response(self) -> Response {
        match self {
            ActivityJsonRejection::InvalidContentType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Expected request with `Content-Type: application/activity+json`",
            )
                .into_response(),
            ActivityJsonRejection::Json(rejection) => rejection.into_response(),
        }
    }
}

impl<T, S> FromRequest<S> for ActivityJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ActivityJsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if is_activity_json_type(req.headers()) {
            let v = Json::<T>::from_request(req, state).await?;
            Ok(ActivityJson(v.0))
        } else {
            Err(ActivityJsonRejection::InvalidContentType)?
        }
    }
}
