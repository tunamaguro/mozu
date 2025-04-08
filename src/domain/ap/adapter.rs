use crate::domain::account::model::AccountId;

use super::model::{
    ActorRow, CreateActorError, CreateLocalActorError, CreateLocalActorRequest,
    CreateRemoteActorError, CreateRemoteActorRequest, LocalActor, RemoteActor,
    actor::{FindActorError, FindRemoteActorRequest},
    note::{
        CreateLocalNoteError, CreateLocalNoteRequest, CreateRemoteNoteError,
        CreateRemoteNoteRequest, LocalNote, RemoteNote,
    },
};

#[async_trait::async_trait]
pub trait ApService: Send + Sync + 'static {
    async fn create_local_actor(
        &self,
        req: CreateLocalActorRequest,
    ) -> Result<LocalActor, CreateLocalActorError>;
    async fn create_remote_actor(
        &self,
        req: CreateRemoteActorRequest,
    ) -> Result<RemoteActor, CreateRemoteActorError>;

    async fn create_local_note(
        &self,
        req: CreateLocalNoteRequest,
    ) -> Result<LocalNote, CreateLocalNoteError>;

    async fn create_remote_note(
        &self,
        req: CreateRemoteNoteRequest,
    ) -> Result<RemoteNote, CreateRemoteNoteError>;
}

#[async_trait::async_trait]
pub trait ActorRepository: Send + Sync + 'static {
    async fn upsert_actor(&self, req: ActorRow) -> Result<ActorRow, CreateActorError>;
    async fn find_local_actor(&self, account_id: &AccountId) -> Result<ActorRow, FindActorError>;
    async fn find_remote_actor(
        &self,
        req: &FindRemoteActorRequest,
    ) -> Result<ActorRow, FindActorError>;
}

#[async_trait::async_trait]
pub trait NoteRepository: Send + Sync + 'static {
    async fn create_local_note(&self, req: LocalNote) -> Result<LocalNote, CreateLocalNoteError>;

    async fn create_remote_note(
        &self,
        req: RemoteNote,
    ) -> Result<RemoteNote, CreateRemoteNoteError>;
}
