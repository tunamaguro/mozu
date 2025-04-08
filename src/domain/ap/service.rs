use crate::{
    ap::ActorType,
    domain::{
        ap::model::{ActorId, ActorRow},
        hosturl::HostUrlService,
    },
};

use super::{
    adapter::{ActorRepository, ApService, NoteRepository},
    model::{
        CreateLocalActorError, CreateLocalActorRequest, CreateRemoteActorError,
        CreateRemoteActorRequest, LocalActor, RemoteActor,
        actor::FindRemoteActorRequest,
        note::{
            CreateLocalNoteError, CreateLocalNoteRequest, CreateRemoteNoteError,
            CreateRemoteNoteRequest, LocalNote, NoteId, RemoteNote,
        },
    },
};

#[derive(Debug, Clone)]
pub struct Service<AR, NR, H> {
    actor_repo: AR,
    note_repo: NR,
    host_url: H,
}

impl<AR, NR, H> Service<AR, NR, H>
where
    AR: ActorRepository,
    NR: NoteRepository,
    H: HostUrlService,
{
    pub fn new(actor_repo: AR, note_repo: NR, host_url: H) -> Self {
        Self {
            actor_repo,
            note_repo,
            host_url,
        }
    }
}

#[async_trait::async_trait]
impl<AR, NR, H> ApService for Service<AR, NR, H>
where
    AR: ActorRepository,
    NR: NoteRepository,
    H: HostUrlService,
{
    async fn create_local_actor(
        &self,
        req: CreateLocalActorRequest,
    ) -> Result<LocalActor, CreateLocalActorError> {
        let CreateLocalActorRequest { account_id, name } = req;

        let inbox_url = self.host_url.inbox_url(name.as_str());
        let outbox_url = self.host_url.outbox_url(name.as_str());
        let actor_url = self.host_url.actor_url(name.as_str());
        let shared_inbox_url = self.host_url.shared_inbox_url();
        let row = ActorRow {
            id: ActorId::new(),
            actor_type: ActorType::Person,
            name: name.as_str().to_string(),
            inbox_url,
            outbox_url,
            actor_url,
            account_id: account_id.into(),
            shared_inbox_url: shared_inbox_url.into(),
        };
        let actor_row = self.actor_repo.upsert_actor(row).await?;

        let local_actor = LocalActor::try_from(actor_row)?;

        Ok(local_actor)
    }

    async fn create_remote_actor(
        &self,
        req: CreateRemoteActorRequest,
    ) -> Result<RemoteActor, CreateRemoteActorError> {
        let CreateRemoteActorRequest {
            actor_type,
            actor_url,
            inbox_url,
            name,
            outbox_url,
            shared_inbox_url,
        } = req;

        let actor_row = ActorRow {
            id: ActorId::new(),
            actor_type,
            name,
            inbox_url,
            outbox_url,
            actor_url,
            account_id: None,
            shared_inbox_url,
        };
        let actor_row = self.actor_repo.upsert_actor(actor_row).await?;
        let remote_actor = RemoteActor::from(actor_row);
        Ok(remote_actor)
    }

    async fn create_local_note(
        &self,
        req: CreateLocalNoteRequest,
    ) -> Result<LocalNote, CreateLocalNoteError> {
        let actor = self.actor_repo.find_local_actor(&req.account_id).await?;

        let note_id = NoteId::new();
        let note_url = self.host_url.note_url(&note_id.to_string());

        let note = LocalNote {
            id: note_id,
            actor_id: actor.id,
            account_id: req.account_id,
            content: req.content,
            note_url,
        };

        let note = self.note_repo.create_local_note(note).await?;
        Ok(note)
    }

    async fn create_remote_note(
        &self,
        req: CreateRemoteNoteRequest,
    ) -> Result<RemoteNote, CreateRemoteNoteError> {
        let remote_actor_req = FindRemoteActorRequest {
            name: req.name,
            host: req.host,
        };
        let actor = self.actor_repo.find_remote_actor(&remote_actor_req).await?;
        let note_id = NoteId::new();

        let remote_note = RemoteNote {
            id: note_id,
            actor_id: actor.id,
            content: req.content,
            note_url: req.note_url,
        };
        let note = self.note_repo.create_remote_note(remote_note).await?;
        Ok(note)
    }
}
