use crate::domain::ap::{adapter::ApService, model::CreateLocalActorRequest};

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
    AP: ApService,
{
    pub fn new(repo: R, ap: AP) -> Self {
        Self { repo, ap }
    }
}

#[async_trait::async_trait]
impl<R, AP> AccountService for Service<R, AP>
where
    R: AccountRepository,
    AP: ApService,
{
    #[tracing::instrument(skip(self))]
    async fn create(&self, req: CreateAccountRequest) -> Result<Account, CreateAccountError> {
        tracing::info!("Creating account");
        let CreateAccountRequest { name } = req;
        let account = Account::new(name);
        let created_account = self.repo.create(account).await?;

        tracing::info!("Creating actor");

        let local_actor_req = CreateLocalActorRequest {
            account_id: created_account.id().clone(),
            name: created_account.name().clone(),
        };

        let _ = self
            .ap
            .create_local_actor(local_actor_req)
            .await
            .inspect_err(|e| tracing::error!(error=%e,"Failed create local actor"))
            .map_err(|e| anyhow::anyhow!(e))?;

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
