mod queries;
use anyhow::Context as _;
use std::str::FromStr;

use deadpool_postgres::{Manager, ManagerConfig, Object, Pool, RecyclingMethod, Transaction};

#[derive(Clone)]
pub struct Postgres {
    pool: Pool,
}

impl Postgres {
    #[tracing::instrument(skip_all)]
    pub async fn new(pg_config: tokio_postgres::Config) -> Result<Self, anyhow::Error> {
        tracing::info!("Connect postgres...");
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

        tracing::info!("Connected to postgres");

        Ok(Self { pool })
    }

    #[tracing::instrument(skip_all)]
    pub async fn from_str(path: &str) -> Result<Self, anyhow::Error> {
        let pg_config = tokio_postgres::Config::from_str(path)?;
        Self::new(pg_config).await
    }

    #[tracing::instrument(skip_all)]
    pub async fn from_env() -> Result<Self, anyhow::Error> {
        let host = std::env::var("DATABASE_HOST")?;
        let port = std::env::var("DATABASE_PORT")?;
        let user = std::env::var("DATABASE_USER")?;
        let password = std::env::var("DATABASE_PASSWORD")?;
        let dbname = std::env::var("DATABASE_NAME")?;

        let mut cfg = tokio_postgres::Config::new();
        cfg.host(host.as_str())
            .port(port.parse::<u16>()?)
            .user(user.as_str())
            .password(password.as_str())
            .dbname(dbname.as_str());

        Self::new(cfg.to_owned()).await
    }

    async fn get_client(&self) -> Result<Object, anyhow::Error> {
        self.pool
            .get()
            .await
            .context("failed to get client from pool")
    }
}

async fn get_transaction(client: &mut Object) -> Result<Transaction, anyhow::Error> {
    client
        .transaction()
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .inspect_err(|e| tracing::error!(error = %e, "Failed to get transaction"))
}

mod account_repository_impl {
    use crate::domain::account::{
        adapter::AccountRepository,
        model::{Account, AccountId, AccountName, CreateAccountError, FindAccountError},
    };

    use super::*;
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
        async fn find_by_name(
            &self,
            name: &AccountName,
        ) -> Result<Option<Account>, FindAccountError> {
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
}

mod actor_repository_impl {
    use crate::{
        ap::ActorType,
        domain::{
            HttpUrl, HttpUrlError,
            account::model::AccountId,
            ap::{
                adapter::ActorRepository,
                model::{ActorId, ActorRow, CreateActorError, actor::FindActorError},
            },
        },
    };

    use super::*;

    impl From<ActorType> for queries::ActorType {
        fn from(actor_type: ActorType) -> Self {
            match actor_type {
                ActorType::Person => queries::ActorType::Person,
                ActorType::Application => queries::ActorType::Application,
                ActorType::Service => queries::ActorType::Service,
                ActorType::Group => queries::ActorType::Group,
                ActorType::Organization => queries::ActorType::Organization,
            }
        }
    }

    impl From<queries::ActorType> for ActorType {
        fn from(actor_type: queries::ActorType) -> Self {
            match actor_type {
                queries::ActorType::Person => ActorType::Person,
                queries::ActorType::Application => ActorType::Application,
                queries::ActorType::Service => ActorType::Service,
                queries::ActorType::Group => ActorType::Group,
                queries::ActorType::Organization => ActorType::Organization,
            }
        }
    }

    impl From<deadpool_postgres::tokio_postgres::Error> for FindActorError {
        fn from(e: deadpool_postgres::tokio_postgres::Error) -> Self {
            tracing::error!(error = %e, "Failed to find actor");
            Self::DataBaseError(e.into())
        }
    }

    impl From<HttpUrlError> for FindActorError {
        fn from(e: HttpUrlError) -> Self {
            Self::InvalidData(anyhow::anyhow!(e).context("expected valid url"))
        }
    }

    impl TryFrom<queries::GetActorRow> for ActorRow {
        type Error = FindActorError;
        fn try_from(row: queries::GetActorRow) -> Result<Self, Self::Error> {
            let id = row.actors_id.into();
            let actor_type = row.actors_type.into();
            let actor_url = HttpUrl::from_str(&row.actors_actor_url)?;
            let inbox_url = HttpUrl::from_str(&row.actors_inbox_url)?;
            let outbox_url = HttpUrl::from_str(&row.actors_outbox_url)?;
            let shared_inbox_url = row
                .actors_shared_inbox_url
                .map(|url| HttpUrl::from_str(&url))
                .transpose()?;

            let account_id = row.actors_account_id.map(|id| id.into());

            Ok(Self {
                id,
                actor_type,
                name: row.actors_name,
                actor_url,
                inbox_url,
                outbox_url,
                shared_inbox_url,
                account_id,
            })
        }
    }

    impl TryFrom<queries::GetActorByActorUrlRow> for ActorRow {
        type Error = FindActorError;
        fn try_from(row: queries::GetActorByActorUrlRow) -> Result<Self, Self::Error> {
            let id = row.actors_id.into();
            let actor_type = row.actors_type.into();
            let actor_url = HttpUrl::from_str(&row.actors_actor_url)?;
            let inbox_url = HttpUrl::from_str(&row.actors_inbox_url)?;
            let outbox_url = HttpUrl::from_str(&row.actors_outbox_url)?;
            let shared_inbox_url = row
                .actors_shared_inbox_url
                .map(|url| HttpUrl::from_str(&url))
                .transpose()?;

            let account_id = row.actors_account_id.map(|id| id.into());

            Ok(Self {
                id,
                actor_type,
                name: row.actors_name,
                actor_url,
                inbox_url,
                outbox_url,
                shared_inbox_url,
                account_id,
            })
        }
    }

    impl TryFrom<queries::GetActorByNameAndHostRow> for ActorRow {
        type Error = FindActorError;
        fn try_from(row: queries::GetActorByNameAndHostRow) -> Result<Self, Self::Error> {
            let id = row.actors_id.into();
            let actor_type = row.actors_type.into();
            let actor_url = HttpUrl::from_str(&row.actors_actor_url)?;
            let inbox_url = HttpUrl::from_str(&row.actors_inbox_url)?;
            let outbox_url = HttpUrl::from_str(&row.actors_outbox_url)?;
            let shared_inbox_url = row
                .actors_shared_inbox_url
                .map(|url| HttpUrl::from_str(&url))
                .transpose()?;

            let account_id = row.actors_account_id.map(|id| id.into());

            Ok(Self {
                id,
                actor_type,
                name: row.actors_name,
                actor_url,
                inbox_url,
                outbox_url,
                shared_inbox_url,
                account_id,
            })
        }
    }

    impl TryFrom<queries::GetActorByAccountIdRow> for ActorRow {
        type Error = FindActorError;
        fn try_from(row: queries::GetActorByAccountIdRow) -> Result<Self, Self::Error> {
            let id = row.actors_id.into();
            let actor_type = row.actors_type.into();
            let actor_url = HttpUrl::from_str(&row.actors_actor_url)?;
            let inbox_url = HttpUrl::from_str(&row.actors_inbox_url)?;
            let outbox_url = HttpUrl::from_str(&row.actors_outbox_url)?;
            let shared_inbox_url = row
                .actors_shared_inbox_url
                .map(|url| HttpUrl::from_str(&url))
                .transpose()?;
            let account_id =
                row.actors_account_id
                    .ok_or(FindActorError::InvalidData(anyhow::anyhow!(
                        "expected account id"
                    )))?;

            Ok(Self {
                id,
                actor_type,
                name: row.actors_name,
                actor_url,
                inbox_url,
                outbox_url,
                shared_inbox_url,
                account_id: Some(account_id.into()),
            })
        }
    }

    impl ActorRepository for Postgres {
        async fn create(&self, actor: ActorRow) -> Result<ActorRow, (ActorRow, CreateActorError)> {
            let client = self.get_client().await;
            let client = match client {
                Ok(client) => client,
                Err(e) => {
                    return Err((actor, CreateActorError::DataBaseError(e)));
                }
            };

            let actor_type = queries::ActorType::from(actor.actor_type);
            let shared_inbox_url = actor.shared_inbox_url.as_ref().map(|url| url.as_str());
            let account_id = actor.account_id.as_ref().map(|id| id.as_ref());

            let res = queries::create_actor(
                &client,
                &actor.id,
                &actor_type,
                &actor.name,
                actor.host(),
                actor.actor_url.as_str(),
                actor.inbox_url.as_str(),
                actor.outbox_url.as_str(),
                shared_inbox_url,
                account_id,
            )
            .await;

            match res {
                Ok(_) => Ok(actor),

                Err(e) => {
                    tracing::error!(error = %e, "Failed to create actor");
                    Err((
                        actor,
                        CreateActorError::DataBaseError(
                            anyhow::Error::from(e).context("insert failed"),
                        ),
                    ))
                }
            }
        }
        async fn find_by_id(&self, id: &ActorId) -> Result<Option<ActorRow>, FindActorError> {
            let client = self
                .get_client()
                .await
                .map_err(FindActorError::DataBaseError)?;
            let get_actor_row = queries::get_actor(&client, id).await?;

            match get_actor_row {
                Some(row) => {
                    let actor_row = ActorRow::try_from(row)?;
                    Ok(Some(actor_row))
                }
                None => Ok(None),
            }
        }
        async fn find_by_url(&self, url: &HttpUrl) -> Result<Option<ActorRow>, FindActorError> {
            let client = self
                .get_client()
                .await
                .map_err(FindActorError::DataBaseError)?;

            let row = queries::get_actor_by_actor_url(&client, url.as_str()).await?;

            match row {
                Some(row) => {
                    let actor_row = ActorRow::try_from(row)?;
                    Ok(Some(actor_row))
                }
                None => Ok(None),
            }
        }
        async fn find_by_host_name(
            &self,
            host: &str,
            name: &str,
        ) -> Result<Option<ActorRow>, FindActorError> {
            let client = self
                .get_client()
                .await
                .map_err(FindActorError::DataBaseError)?;

            let row = queries::get_actor_by_name_and_host(&client, host, name).await?;

            match row {
                Some(row) => {
                    let actor_row = ActorRow::try_from(row)?;
                    Ok(Some(actor_row))
                }
                None => Ok(None),
            }
        }
        async fn find_by_account_id(
            &self,
            account_id: &AccountId,
        ) -> Result<Option<ActorRow>, FindActorError> {
            let client = self
                .get_client()
                .await
                .map_err(FindActorError::DataBaseError)?;

            let row = queries::get_actor_by_account_id(&client, Some(account_id)).await?;

            match row {
                Some(row) => {
                    let actor_row = ActorRow::try_from(row)?;
                    Ok(Some(actor_row))
                }
                None => Ok(None),
            }
        }
    }
}
