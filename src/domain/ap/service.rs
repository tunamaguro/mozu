use super::{
    adapter::{ActorRepository, ActorService, WebFingerPort},
    model::{
        ActorId, CreateActorError, LocalActor,
        actor::{Actor, FindActorError, ResolveActorError, ResolveActorRequest},
    },
};
use crate::domain::{
    HttpUrl,
    account::{adapter::AccountRepository, model::AccountId},
    ap::model::ActorRow,
    hosturl::HostUrlService,
};

#[derive(Debug, Clone)]
pub struct Service<
    ActorRepo: ActorRepository,
    AccountRepo: AccountRepository,
    WF: WebFingerPort,
    HS: HostUrlService,
> {
    actor_repository: ActorRepo,
    account_repository: AccountRepo,
    webfinger: WF,
    host_url: HS,
}

impl<
    ActorRepo: ActorRepository,
    AccountRepo: AccountRepository,
    WF: WebFingerPort,
    HS: HostUrlService,
> Service<ActorRepo, AccountRepo, WF, HS>
{
    pub fn new(
        actor_repository: ActorRepo,
        account_repository: AccountRepo,
        webfinger: WF,
        host_url: HS,
    ) -> Self {
        Self {
            actor_repository,
            account_repository,
            webfinger,
            host_url,
        }
    }
}

#[async_trait::async_trait]
impl<
    ActorRepo: ActorRepository,
    AccountRepo: AccountRepository,
    WF: WebFingerPort,
    HS: HostUrlService,
> ActorService for Service<ActorRepo, AccountRepo, WF, HS>
{
    #[tracing::instrument(skip(self))]
    async fn create_local_actor(
        &self,
        account_id: &AccountId,
    ) -> Result<LocalActor, CreateActorError> {
        let account = self.account_repository.find_by_id(account_id).await;
        let account = match account {
            Ok(Some(v)) => v,
            Ok(None) => return Err(CreateActorError::AccountNotExists),
            Err(e) => return Err(CreateActorError::DataBaseError(e.into())),
        };

        let local_actor = LocalActor {
            id: Default::default(),
            actor_type: crate::ap::ActorType::Person,
            name: account.name().as_str().to_string(),
            actor_url: self.host_url.actor_url(account.name().as_str()),
            inbox_url: self.host_url.inbox_url(account.name().as_str()),
            outbox_url: self.host_url.outbox_url(account.name().as_str()),
            shared_inbox_url: self.host_url.shared_inbox_url(),
            account_id: account.id().clone(),
        };

        let row = self
            .actor_repository
            .create(local_actor.into())
            .await
            .unwrap();
        let any_actor = Actor::from(row);
        match any_actor {
            Actor::Local(local_actor) => Ok(local_actor),
            Actor::Remote(_) => {
                return Err(CreateActorError::DataBaseError(anyhow::anyhow!(
                    "expect local actor"
                )));
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn find_actor_by_id(&self, id: &ActorId) -> Result<Option<Actor>, FindActorError> {
        let row = self.actor_repository.find_by_id(id).await?;
        let actor = row.map(Actor::from);
        Ok(actor)
    }

    #[tracing::instrument(skip(self))]
    async fn resolve_actor_by_url(&self, actor_id: &HttpUrl) -> Result<Actor, ResolveActorError> {
        let db_actor = self.actor_repository.find_by_url(actor_id).await;
        match db_actor {
            Ok(Some(row)) => {
                tracing::info!(actor_id = %actor_id,"actor found in db");
                let actor = Actor::from(row);
                return Ok(actor);
            }
            Ok(None) => {
                tracing::info!(actor_id = %actor_id,"actor not found in db, try to resolve it");
            }
            Err(e) => {
                tracing::error!(actor_id = %actor_id, e = %e, "failed to find actor");
                return Err(ResolveActorError::DataBaseError(e.into()));
            }
        };

        let ap_actor = self.webfinger.lookup_by_id(actor_id).await?;
        let remote_actor = ActorRow {
            id: Default::default(),
            actor_type: ap_actor.kind,
            name: ap_actor.preferred_username,
            actor_url: ap_actor.id,
            inbox_url: ap_actor.inbox,
            outbox_url: ap_actor.outbox,
            shared_inbox_url: None,
            account_id: None,
        };
        let actor_row = self.actor_repository.create(remote_actor).await;
        let actor_row = match actor_row {
            Ok(row) => row,
            Err((_, e)) => {
                tracing::error!(actor_id = %actor_id, e = %e, "failed to create actor");
                return Err(ResolveActorError::DataBaseError(e.into()));
            }
        };
        let actor = Actor::from(actor_row);
        Ok(actor)
    }

    #[tracing::instrument(skip(self))]
    async fn resolve_actor(&self, req: &ResolveActorRequest) -> Result<Actor, ResolveActorError> {
        let db_actor = self
            .actor_repository
            .find_by_host_name(req.host.as_str(), req.name.as_str())
            .await;
        match db_actor {
            Ok(Some(row)) => {
                tracing::info!(actor_id = %req.name, "actor found in db");
                let actor = Actor::from(row);
                return Ok(actor);
            }
            Ok(None) => {
                tracing::info!(actor_id = %req.name, "actor not found in db, try to resolve it");
            }
            Err(e) => {
                tracing::error!(actor_id = %req.name, e = %e, "failed to find actor");
                return Err(ResolveActorError::DataBaseError(e.into()));
            }
        };
        let ap_actor = self
            .webfinger
            .lookup_by_host_name(req.host.as_str(), req.name.as_str())
            .await?;
        let remote_actor = ActorRow {
            id: Default::default(),
            actor_type: ap_actor.kind,
            name: ap_actor.preferred_username,
            actor_url: ap_actor.id,
            inbox_url: ap_actor.inbox,
            outbox_url: ap_actor.outbox,
            shared_inbox_url: None,
            account_id: None,
        };
        let actor_row = self.actor_repository.create(remote_actor).await;
        let actor_row = match actor_row {
            Ok(row) => row,
            Err((_, e)) => {
                tracing::error!(actor_id = %req.name, e = %e, "failed to create actor");
                return Err(ResolveActorError::DataBaseError(e.into()));
            }
        };
        let actor = Actor::from(actor_row);
        Ok(actor)
    }
}
