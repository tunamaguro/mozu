use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing};
use serde::{Deserialize, Serialize};

use crate::domain::{
    account::model::AccountId,
    ap::model::note::{CreateLocalNoteError, CreateLocalNoteRequest, LocalNote, NoteId},
};

use super::state::{AppRegistry, AppRegistryExt};

#[derive(Debug, Serialize)]
pub struct CreatePostSuccess {
    note_id: NoteId,
    content: String,
}

impl IntoResponse for CreatePostSuccess {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

impl From<LocalNote> for CreatePostSuccess {
    fn from(value: LocalNote) -> Self {
        CreatePostSuccess {
            note_id: value.id,
            content: value.content,
        }
    }
}

pub enum CreatePostError {
    ActorNotFound,
    InteranalServerError,
}

impl IntoResponse for CreatePostError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CreatePostError::ActorNotFound => {
                (StatusCode::FORBIDDEN, "actor not found").into_response()
            }
            CreatePostError::InteranalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

impl From<CreateLocalNoteError> for CreatePostError {
    fn from(value: CreateLocalNoteError) -> Self {
        match value {
            CreateLocalNoteError::ActorNotFound => CreatePostError::ActorNotFound,
            CreateLocalNoteError::Unknown(_) => CreatePostError::InteranalServerError,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    account_id: AccountId,
    content: String,
}

impl TryFrom<CreatePostRequest> for CreateLocalNoteRequest {
    type Error = std::convert::Infallible;
    fn try_from(value: CreatePostRequest) -> Result<Self, Self::Error> {
        Ok(CreateLocalNoteRequest {
            account_id: value.account_id,
            content: value.content,
        })
    }
}

#[tracing::instrument(skip(registry))]
pub async fn create_post(
    State(registry): State<AppRegistry>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<CreatePostSuccess, CreatePostError> {
    let req = payload.try_into().unwrap();

    let ap_service = registry.ap_service();
    let note = ap_service.create_local_note(req).await?;

    Ok(note.into())
}

pub fn router(registry: AppRegistry) -> Router {
    Router::new()
        .route("/", routing::post(create_post))
        .with_state(registry)
}
