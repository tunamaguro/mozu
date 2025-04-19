use crate::{
    ap::ActorType,
    domain::{HttpUrl, Id, account::model::AccountId},
};

pub type ActorId = Id<ActorRow>;

#[derive(Debug, Clone)]
pub struct ActorRow {
    pub id: ActorId,
    pub actor_type: ActorType,
    /// actor name
    pub name: String,
    /// actor url
    pub actor_url: HttpUrl,
    /// actor inbox`
    pub inbox_url: HttpUrl,
    /// actor outbox
    pub outbox_url: HttpUrl,
    /// actor shared inbox
    pub shared_inbox_url: Option<HttpUrl>,
    /// account id
    pub account_id: Option<AccountId>,
}

impl ActorRow {
    pub fn host(&self) -> &str {
        self.actor_url.host()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateActorError {
    #[error("actor is already exists")]
    AlreadyExists,
    #[error("account is not exists")]
    AccountNotExists,
    #[error(transparent)]
    DataBaseError(anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct LocalActor {
    pub id: ActorId,
    pub actor_type: ActorType,
    /// actor name
    pub name: String,
    /// actor url
    pub actor_url: HttpUrl,
    /// actor inbox`
    pub inbox_url: HttpUrl,
    /// actor outbox
    pub outbox_url: HttpUrl,
    /// actor shared inbox
    pub shared_inbox_url: HttpUrl,
    /// account id
    pub account_id: AccountId,
}

impl From<LocalActor> for ActorRow {
    fn from(actor: LocalActor) -> Self {
        Self {
            id: actor.id,
            actor_type: actor.actor_type,
            name: actor.name,
            actor_url: actor.actor_url,
            inbox_url: actor.inbox_url,
            outbox_url: actor.outbox_url,
            shared_inbox_url: Some(actor.shared_inbox_url),
            account_id: Some(actor.account_id),
        }
    }
}

pub struct CreateLocalActorRequest {
    pub account_id: AccountId,
}

#[derive(Debug, Clone)]
pub struct RemoteActor {
    id: ActorId,
    actor_type: ActorType,
    /// actor name
    pub name: String,
    /// actor url
    pub actor_url: HttpUrl,
    /// actor inbox`
    pub inbox_url: HttpUrl,
    /// actor outbox
    pub outbox_url: HttpUrl,
    /// actor shared inbox
    pub shared_inbox_url: Option<HttpUrl>,
}

impl From<RemoteActor> for ActorRow {
    fn from(actor: RemoteActor) -> Self {
        Self {
            id: actor.id,
            actor_type: actor.actor_type,
            name: actor.name,
            actor_url: actor.actor_url,
            inbox_url: actor.inbox_url,
            outbox_url: actor.outbox_url,
            shared_inbox_url: actor.shared_inbox_url,
            account_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Actor {
    Local(LocalActor),
    Remote(RemoteActor),
}

impl Actor {
    pub fn as_local(&self) -> Option<&LocalActor> {
        match self {
            Actor::Local(actor) => Some(actor),
            Actor::Remote(_) => None,
        }
    }

    pub fn as_remote(&self) -> Option<&RemoteActor> {
        match self {
            Actor::Local(_) => None,
            Actor::Remote(actor) => Some(actor),
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Actor::Local(_))
    }

    pub fn is_remote(&self) -> bool {
        matches!(self, Actor::Remote(_))
    }
}

impl From<ActorRow> for Actor {
    fn from(row: ActorRow) -> Self {
        let ActorRow {
            id,
            actor_type,
            name,
            actor_url,
            inbox_url,
            outbox_url,
            shared_inbox_url,
            account_id,
        } = row;

        match (account_id, shared_inbox_url) {
            (Some(account_id), Some(shared_inbox_url)) => Actor::Local(LocalActor {
                id,
                actor_type,
                name,
                actor_url,
                inbox_url,
                outbox_url,
                shared_inbox_url,
                account_id,
            }),
            (_, shared_inbox_url) => Actor::Remote(RemoteActor {
                id,
                actor_type,
                name,
                actor_url,
                inbox_url,
                outbox_url,
                shared_inbox_url,
            }),
        }
    }
}

impl From<Actor> for ActorRow {
    fn from(actor: Actor) -> Self {
        match actor {
            Actor::Local(actor) => ActorRow::from(actor),
            Actor::Remote(actor) => ActorRow::from(actor),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FindActorError {
    #[error("actor not found")]
    NotFound,
    #[error(transparent)]
    InvalidData(anyhow::Error),
    #[error(transparent)]
    DataBaseError(anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct ResolveActorRequest {
    /// actor host name
    pub host: String,
    /// actor preferred name
    pub name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ResolveActorError {
    #[error("{0}")]
    WebFingerError(#[from] WebFingerError),
    #[error(transparent)]
    DataBaseError(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum WebFingerError {
    #[error("unexpected status code: {0}")]
    HttpStatusError(u16),
    #[error("{0}")]
    UnexpectedResponse(String),
    #[error("invalid http signature")]
    InvalidSignature,
    #[error("webfinger response is missing self link")]
    MissingSelfLink,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
