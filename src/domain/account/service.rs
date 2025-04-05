use super::{
    adapter::{AccountRepository, AccountService},
    model::{
        Account, AccountId, AccountName, CreateAccountError, CreateAccountRequest, FindAccountError,
    },
};

#[derive(Debug, Clone)]
pub struct Service<R> {
    repo: R,
}

impl<R> Service<R>
where
    R: AccountRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R> AccountService for Service<R>
where
    R: AccountRepository,
{
    async fn create(&self, req: CreateAccountRequest) -> Result<Account, CreateAccountError> {
        let CreateAccountRequest { name } = req;
        let account = Account::new(name);
        let created_account = self.repo.create(account).await?;
        Ok(created_account)
    }
    async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, FindAccountError> {
        let account = self.repo.find_by_id(id).await?;
        Ok(account)
    }
    async fn find_by_name(&self, name: &AccountName) -> Result<Option<Account>, FindAccountError> {
        let account = self.repo.find_by_name(name).await?;
        Ok(account)
    }
}
