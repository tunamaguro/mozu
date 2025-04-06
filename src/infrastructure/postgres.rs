mod queries;
use std::str::FromStr;

use deadpool_postgres::{Manager, ManagerConfig, Object, Pool, RecyclingMethod};

use crate::domain::account::{
    adapter::AccountRepository,
    model::{Account, AccountId, AccountName, CreateAccountError, FindAccountError},
};

pub struct Postgres {
    pool: Pool,
}

impl Postgres {
    pub async fn new(path: &str) -> Result<Self, anyhow::Error> {
        let pg_config = tokio_postgres::Config::from_str(path)?;
        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Verified,
        };
        let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        let pool = Pool::builder(mgr).max_size(4).build()?;

        // Check connection
        let client = pool.get().await.map_err(|e| anyhow::anyhow!(e))?;
        client
            .simple_query("SELECT 1")
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(Self { pool })
    }

    async fn get_client(&self) -> Result<Object, anyhow::Error> {
        self.pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .inspect_err(|e| {
                tracing::error!(error = %e, "Failed to get client from pool");
            })
    }
}

#[async_trait::async_trait]
impl AccountRepository for Postgres {
    #[tracing::instrument(skip(self))]
    async fn create(&self, account: Account) -> Result<Account, CreateAccountError> {
        let client = self.get_client().await?;
        let res = queries::create_account(&client, account.id(), account.name().as_str()).await;
        match res {
            Ok(_) => Ok(account),
            Err(e) if e.is_closed() => Err(CreateAccountError::Unknown(e.into())),
            Err(_) => Err(CreateAccountError::AlreadyExists),
        }
    }
    #[tracing::instrument(skip(self))]

    async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, FindAccountError> {
        let client = self.get_client().await?;
        let result = queries::find_account_by_id(&client, id)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        if let Some(result) = result {
            let name = AccountName::new(&result.accounts_name)?;
            let id = result.accounts_id.into();
            Ok(Some(Account::from_id_name(id, name)))
        } else {
            return Ok(None);
        }
    }
    #[tracing::instrument(skip(self))]
    async fn find_by_name(&self, name: &AccountName) -> Result<Option<Account>, FindAccountError> {
        let client = self.get_client().await?;
        let result = queries::find_account_by_name(&client, name.as_str())
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .inspect_err(|e| {
                tracing::error!(error = %e, "Failed to find account by name");
            })?;

        if let Some(result) = result {
            let name = AccountName::new(&result.accounts_name)?;
            let id = result.accounts_id.into();
            Ok(Some(Account::from_id_name(id, name)))
        } else {
            return Ok(None);
        }
    }
}
