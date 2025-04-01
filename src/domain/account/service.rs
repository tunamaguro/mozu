use super::adapter::{AccountRepository, AccountService};

#[derive(Debug, Clone)]
pub struct Service<R> {
    repo: R,
}

impl <R> Service<R>
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
    async fn create(&self, req: &super::model::CreateAccountRequest) -> Result<super::model::Account, super::model::CreateAccountError> {
        self.repo.create(req).await
    }

    async fn find_by_id(&self, id: &uuid::Uuid) -> Result<Option<super::model::Account>, super::model::FindAccountError> {
        self.repo.find_by_id(id).await
    }

    async fn find_by_name(&self, name: &super::model::AccountName) -> Result<Option<super::model::Account>, super::model::FindAccountError> {
        self.repo.find_by_name(name).await
    }
}