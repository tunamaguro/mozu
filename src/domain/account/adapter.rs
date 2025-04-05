use super::model::{
    Account, AccountId, AccountName, CreateAccountError, CreateAccountRequest, FindAccountError,
};

#[async_trait::async_trait]
pub trait AccountService: Send + Sync + 'static {
    async fn create(&self, req: CreateAccountRequest) -> Result<Account, CreateAccountError>;
    async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, FindAccountError>;
    async fn find_by_name(&self, name: &AccountName) -> Result<Option<Account>, FindAccountError>;
}

#[async_trait::async_trait]
pub trait AccountRepository: Send + Sync + 'static {
    async fn create(&self, account: Account) -> Result<Account, CreateAccountError>;
    async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, FindAccountError>;
    async fn find_by_name(&self, name: &AccountName) -> Result<Option<Account>, FindAccountError>;
}
