use crate::domain::{HttpUrl, Id, account::model::AccountId};

use super::{ActorId, actor::FindActorError};

pub type NoteId = Id<Note>;

#[derive(Debug, Clone)]
pub enum Note {
    Local(LocalNote),
    Remote(RemoteNote),
}

#[derive(Debug, Clone)]
pub struct LocalNote {
    /// note id
    pub(crate) id: NoteId,
    /// who created the note
    pub(crate) account_id: AccountId,
    pub(crate) actor_id: ActorId,
    /// content
    pub(crate) content: String,
    /// note url
    pub(crate) note_url: HttpUrl,
}

#[derive(Debug, Clone)]
pub struct CreateLocalNoteRequest {
    /// who created the note
    pub(crate) account_id: AccountId,
    /// content
    pub(crate) content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateLocalNoteError {
    #[error("Actor not found")]
    ActorNotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<FindActorError> for CreateLocalNoteError {
    fn from(e: FindActorError) -> Self {
        match e {
            FindActorError::NotFound => CreateLocalNoteError::ActorNotFound,
            FindActorError::Unknown(e) => CreateLocalNoteError::Unknown(e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateRemoteNoteRequest {
    pub(crate) name: String,
    pub(crate) host: String,
    pub(crate) content: String,
    pub(crate) note_url: HttpUrl,
}

#[derive(Debug, Clone)]
pub struct RemoteNote {
    /// note id
    pub(crate) id: NoteId,
    pub(crate) actor_id: ActorId,
    /// content
    pub(crate) content: String,
    /// note url
    pub(crate) note_url: HttpUrl,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateRemoteNoteError {
    #[error("Actor not found")]
    ActorNotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<FindActorError> for CreateRemoteNoteError {
    fn from(e: FindActorError) -> Self {
        match e {
            FindActorError::NotFound => CreateRemoteNoteError::ActorNotFound,
            FindActorError::Unknown(e) => CreateRemoteNoteError::Unknown(e),
        }
    }
}
