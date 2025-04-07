use super::model::{
    ActorRow, CreateActorError, CreateLocalActorError, CreateLocalActorRequest,
    CreateRemoteActorError, CreateRemoteActorRequest, LocalActor, RemoteActor,
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
}

#[async_trait::async_trait]
pub trait ApRepository: Send + Sync + 'static {
    async fn upsert_actor(&self, req: ActorRow) -> Result<ActorRow, CreateActorError>;
}
