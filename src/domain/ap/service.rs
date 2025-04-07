use crate::{
    ap::ActorType,
    domain::{
        ap::model::{ActorId, ActorRow},
        hosturl::HostUrlService,
    },
};

use super::{
    adapter::{ApRepository, ApService},
    model::{
        CreateLocalActorError, CreateLocalActorRequest, CreateRemoteActorError,
        CreateRemoteActorRequest, LocalActor, RemoteActor,
    },
};

#[derive(Debug, Clone)]
pub struct Service<R, H> {
    repo: R,
    host_url: H,
}

impl<R, H> Service<R, H>
where
    R: ApRepository,
    H: HostUrlService,
{
    pub fn new(repo: R, host_url: H) -> Self {
        Self { repo, host_url }
    }
}

#[async_trait::async_trait]
impl<R, H> ApService for Service<R, H>
where
    R: ApRepository,
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
        let actor_row = self.repo.upsert_actor(row).await?;

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
        let actor_row = self.repo.upsert_actor(actor_row).await?;
        let remote_actor = RemoteActor::from(actor_row);
        Ok(remote_actor)
    }
}
