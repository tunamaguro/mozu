use crate::domain::{HttpUrl, account::model::AccountId};

use super::model::{
    ActorId, ActorRow, CreateActorError, LocalActor,
    actor::{Actor, FindActorError, ResolveActorError, ResolveActorRequest, WebFingerError},
};

#[async_trait::async_trait]
pub trait ActorService: Send + Sync + 'static {
    /// Creates a new local actor
    async fn create_local_actor(
        &self,
        account_id: &AccountId,
    ) -> Result<LocalActor, CreateActorError>;
    /// Finds an actor by its ID
    async fn find_actor_by_id(&self, id: &ActorId) -> Result<Option<Actor>, FindActorError>;
    /// Resolves an actor by its URL
    async fn resolve_actor_by_url(&self, actor_id: &HttpUrl) -> Result<Actor, ResolveActorError>;
    /// Resolves an actor by its name and host
    async fn resolve_actor(&self, req: &ResolveActorRequest) -> Result<Actor, ResolveActorError>;
}

#[trait_variant::make(ActorRepository:Send)]
pub trait LocalActorRepository: Send + Sync + 'static {
    async fn create(&self, actor: ActorRow) -> Result<ActorRow, (ActorRow, CreateActorError)>;
    async fn find_by_id(&self, id: &ActorId) -> Result<Option<ActorRow>, FindActorError>;
    async fn find_by_url(&self, url: &HttpUrl) -> Result<Option<ActorRow>, FindActorError>;
    async fn find_by_host_name(
        &self,
        host: &str,
        name: &str,
    ) -> Result<Option<ActorRow>, FindActorError>;
    async fn find_by_account_id(
        &self,
        account_id: &AccountId,
    ) -> Result<Option<ActorRow>, FindActorError>;
}

#[trait_variant::make(WebFingerPort:Send)]
pub trait LocalWebFingerPort: Send + Sync + 'static {
    async fn lookup_by_id(&self, actor_id: &HttpUrl) -> Result<crate::ap::Actor, WebFingerError>;
    async fn lookup_by_host_name(
        &self,
        host: &str,
        name: &str,
    ) -> Result<crate::ap::Actor, WebFingerError>;
}
