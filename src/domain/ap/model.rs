use crate::{
    ap::ActorType,
    domain::{
        HttpUrl, Id,
        account::model::{AccountId, AccountName},
    },
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
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct LocalActor {
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
    pub shared_inbox_url: HttpUrl,
    /// account id
    pub account_id: AccountId,
}

#[derive(Debug, thiserror::Error)]
pub enum RowToLocalActorError {
    #[error("local actor must have account id")]
    AccountIdNotFound,
    #[error("local actor must have shared inbox url")]
    SharedInboxNotFound,
}

impl TryFrom<ActorRow> for LocalActor {
    type Error = RowToLocalActorError;

    fn try_from(row: ActorRow) -> Result<Self, Self::Error> {
        let account_id = row
            .account_id
            .ok_or(RowToLocalActorError::AccountIdNotFound)?;
        let shared_inbox_url = row
            .shared_inbox_url
            .ok_or(RowToLocalActorError::SharedInboxNotFound)?;
        Ok(Self {
            id: row.id,
            actor_type: row.actor_type,
            name: row.name,
            actor_url: row.actor_url,
            inbox_url: row.inbox_url,
            outbox_url: row.outbox_url,
            shared_inbox_url,
            account_id,
        })
    }
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

#[derive(Debug, thiserror::Error)]
pub enum CreateLocalActorError {
    #[error("actor is already exists")]
    AlreadyExists,
    #[error("local actor must have account id")]
    AccountIdNotFound,
    #[error("local actor must have shared inbox url")]
    SharedInboxNotFound,
    #[error(transparent)]
    Unknown(anyhow::Error),
}

impl From<RowToLocalActorError> for CreateLocalActorError {
    fn from(err: RowToLocalActorError) -> Self {
        match err {
            RowToLocalActorError::AccountIdNotFound => Self::AccountIdNotFound,
            RowToLocalActorError::SharedInboxNotFound => Self::SharedInboxNotFound,
        }
    }
}

impl From<CreateActorError> for CreateLocalActorError {
    fn from(err: CreateActorError) -> Self {
        match err {
            CreateActorError::AlreadyExists => Self::AlreadyExists,
            CreateActorError::Unknown(err) => Self::Unknown(err),
        }
    }
}

#[derive(Debug)]
pub struct CreateLocalActorRequest {
    pub account_id: AccountId,
    pub name: AccountName,
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

impl From<ActorRow> for RemoteActor {
    fn from(row: ActorRow) -> Self {
        Self {
            id: row.id,
            actor_type: row.actor_type,
            name: row.name,
            actor_url: row.actor_url,
            inbox_url: row.inbox_url,
            outbox_url: row.outbox_url,
            shared_inbox_url: row.shared_inbox_url,
        }
    }
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
pub struct CreateRemoteActorRequest {
    pub actor_type: ActorType,
    pub name: String,
    pub actor_url: HttpUrl,
    pub inbox_url: HttpUrl,
    pub outbox_url: HttpUrl,
    pub shared_inbox_url: Option<HttpUrl>,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateRemoteActorError {
    #[error("actor is already exists")]
    AlreadyExists,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<CreateActorError> for CreateRemoteActorError {
    fn from(err: CreateActorError) -> Self {
        match err {
            CreateActorError::AlreadyExists => Self::AlreadyExists,
            CreateActorError::Unknown(err) => Self::Unknown(err),
        }
    }
}
