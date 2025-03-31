use super::model::{
    Account, AccountName, CreateAccountError, CreateAccountRequest, FindAccountError,
};

pub trait AccountService: Clone + Send + Sync + 'static {
    async fn create(&self, req: &CreateAccountRequest) -> Result<Account, CreateAccountError>;
    async fn find_by_id(&self, id: &uuid::Uuid) -> Result<Option<Account>, FindAccountError>;
    async fn find_by_name(&self, name: &AccountName) -> Result<Option<Account>, FindAccountError>;
}

pub trait AccountRepository: Clone + Send + Sync + 'static {
    async fn create(&self, req: &CreateAccountRequest) -> Result<Account, CreateAccountError>;
    async fn find_by_id(&self, id: &uuid::Uuid) -> Result<Option<Account>, FindAccountError>;
    async fn find_by_name(&self, name: &AccountName) -> Result<Option<Account>, FindAccountError>;
}
