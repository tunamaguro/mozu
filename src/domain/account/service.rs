use crate::domain::ap::adapter::ActorService;

use super::{
    adapter::{AccountRepository, AccountService},
    model::{
        Account, AccountId, AccountName, CreateAccountError, CreateAccountRequest, FindAccountError,
    },
};

#[derive(Debug, Clone)]
pub struct Service<R, AP> {
    repo: R,
    ap: AP,
}

impl<R, AP> Service<R, AP>
where
    R: AccountRepository,
    AP: ActorService,
{
    pub fn new(repo: R, ap: AP) -> Self {
        Self { repo, ap }
    }
}

#[async_trait::async_trait]
impl<R, AP> AccountService for Service<R, AP>
where
    R: AccountRepository,
    AP: ActorService,
{
    #[tracing::instrument(skip(self))]
    async fn create(&self, req: CreateAccountRequest) -> Result<Account, CreateAccountError> {
        tracing::info!("Creating account");
        let CreateAccountRequest { name } = req;
        let account = Account::new(name);
        let created_account = self.repo.create(account).await?;

        tracing::info!("Creating actor");

        let local_actor = self.ap.create_local_actor(created_account.id()).await;
        match local_actor {
            Ok(_) => tracing::info!("Actor created"),
            Err(e) => {
                tracing::error!(error = %e, "Failed to create actor");
                return Err(CreateAccountError::Unknown(e.into()));
            }
        }

        Ok(created_account)
    }

    #[tracing::instrument(skip(self))]
    async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, FindAccountError> {
        let account = self.repo.find_by_id(id).await?;
        Ok(account)
    }

    #[tracing::instrument(skip(self))]
    async fn find_by_name(&self, name: &AccountName) -> Result<Option<Account>, FindAccountError> {
        let account = self.repo.find_by_name(name).await?;
        Ok(account)
    }
}
