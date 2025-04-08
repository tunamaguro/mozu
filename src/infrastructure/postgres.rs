mod queries;
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
            .map_err(|e| anyhow::anyhow!(e))
            .inspect_err(|e| {
                tracing::error!(error = %e, "Failed to get client from pool");
            })
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

mod ap_repository_impl {
    use crate::domain::{
        HttpUrlError,
        account::model::AccountId,
        ap::{
            adapter::ActorRepository,
            model::{
                ActorRow, CreateActorError,
                actor::{FindActorError, FindRemoteActorRequest},
            },
        },
    };

    use super::*;
    impl From<queries::ActorType> for crate::ap::ActorType {
        fn from(actor_type: queries::ActorType) -> Self {
            match actor_type {
                queries::ActorType::Person => crate::ap::ActorType::Person,
                queries::ActorType::Application => crate::ap::ActorType::Application,
                queries::ActorType::Service => crate::ap::ActorType::Service,
                queries::ActorType::Group => crate::ap::ActorType::Group,
                queries::ActorType::Organization => crate::ap::ActorType::Organization,
            }
        }
    }

    impl From<crate::ap::ActorType> for queries::ActorType {
        fn from(actor_type: crate::ap::ActorType) -> Self {
            match actor_type {
                crate::ap::ActorType::Person => queries::ActorType::Person,
                crate::ap::ActorType::Application => queries::ActorType::Application,
                crate::ap::ActorType::Service => queries::ActorType::Service,
                crate::ap::ActorType::Group => queries::ActorType::Group,
                crate::ap::ActorType::Organization => queries::ActorType::Organization,
            }
        }
    }

    impl From<HttpUrlError> for FindActorError {
        fn from(e: HttpUrlError) -> Self {
            tracing::error!(error = %e, "expected database url to be valid but got invalid url");
            FindActorError::Unknown(e.into())
        }
    }

    #[async_trait::async_trait]
    impl ActorRepository for Postgres {
        async fn upsert_actor(&self, mut actor: ActorRow) -> Result<ActorRow, CreateActorError> {
            let actor_type = actor.actor_type.into();
            let client = self.get_client().await?;

            let shared_inbox_url = actor.shared_inbox_url.as_ref().map(|s| s.as_str());
            let account_id = actor.account_id.as_ref().map(|s| s.as_ref());

            let res = queries::upsert_actor(
                &client,
                &actor.id,
                &actor_type,
                actor.name.as_str(),
                actor.host(),
                actor.actor_url.as_str(),
                actor.inbox_url.as_str(),
                actor.outbox_url.as_str(),
                shared_inbox_url,
                account_id,
            )
            .await;

            match res {
                Ok(Some(row)) => {
                    actor.id = row.actors_id.into();
                    Ok(actor)
                }
                Ok(None) => {
                    tracing::error!("Failed to upsert actor");
                    Err(CreateActorError::AlreadyExists)
                }
                Err(e) if e.is_closed() => Err(CreateActorError::Unknown(e.into())),
                Err(_) => Err(CreateActorError::AlreadyExists),
            }
        }

        async fn find_local_actor(
            &self,
            account_id: &AccountId,
        ) -> Result<ActorRow, FindActorError> {
            let client = self.get_client().await?;
            let result = queries::get_account_actor(&client, Some(account_id)).await;
            match result {
                Ok(Some(row)) => {
                    let actor_row = ActorRow {
                        id: row.actors_id.into(),
                        actor_type: row.actors_type.into(),
                        name: row.actors_name,
                        inbox_url: row.actors_inbox_url.parse()?,
                        outbox_url: row.actors_outbox_url.parse()?,
                        actor_url: row.actors_actor_url.parse()?,
                        account_id: Some(account_id.clone()),
                        shared_inbox_url: row
                            .actors_shared_inbox_url
                            .map(|s| s.parse())
                            .transpose()?,
                    };

                    Ok(actor_row)
                }
                Ok(None) => {
                    tracing::info!(account_id = %account_id, "Actor not found");
                    return Err(FindActorError::NotFound);
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to find local actor");
                    return Err(FindActorError::Unknown(e.into()));
                }
            }
        }
        async fn find_remote_actor(
            &self,
            req: &FindRemoteActorRequest,
        ) -> Result<ActorRow, FindActorError> {
            let client = self.get_client().await?;
            let result = queries::get_actor_by_name_and_host(&client, &req.name, &req.host).await;
            match result {
                Ok(Some(row)) => {
                    let actor_row = ActorRow {
                        id: row.actors_id.into(),
                        actor_type: row.actors_type.into(),
                        name: row.actors_name,
                        inbox_url: row.actors_inbox_url.parse()?,
                        outbox_url: row.actors_outbox_url.parse()?,
                        actor_url: row.actors_actor_url.parse()?,
                        account_id: None,
                        shared_inbox_url: None,
                    };

                    Ok(actor_row)
                }
                Ok(None) => {
                    tracing::info!(name = %req.name, host = %req.host, "Actor not found");
                    return Err(FindActorError::NotFound);
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to find remote actor");
                    return Err(FindActorError::Unknown(e.into()));
                }
            }
        }
    }
}

mod note_repository_impl {
    use super::*;
    use crate::domain::ap::{
        adapter::NoteRepository,
        model::note::{CreateLocalNoteError, CreateRemoteNoteError, LocalNote, RemoteNote},
    };

    #[async_trait::async_trait]
    impl NoteRepository for Postgres {
        async fn create_local_note(
            &self,
            req: LocalNote,
        ) -> Result<LocalNote, CreateLocalNoteError> {
            let mut client = self.get_client().await?;
            let transaction = get_transaction(&mut client).await?;
            let note_source =
                queries::insert_note_source(&transaction, &req.id, &req.account_id, &req.content)
                    .await;
            let note_source = match note_source {
                Ok(Some(row)) => row,
                Ok(None) => {
                    let e = anyhow::anyhow!("Insert success but no row returned");
                    tracing::error!(e = %e,"Failed to insert note source");
                    return Err(CreateLocalNoteError::Unknown(e));
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to insert note source");
                    return Err(CreateLocalNoteError::Unknown(e.into()));
                }
            };

            let result = queries::insert_note(
                &transaction,
                &req.id,
                &req.actor_id,
                Some(&note_source.note_sources_id),
                &req.content,
                req.note_url.as_str(),
            )
            .await;
            if let Err(e) = result {
                tracing::error!(error = %e, "Failed to insert note");
                return Err(CreateLocalNoteError::Unknown(e.into()));
            };

            match transaction.commit().await {
                Ok(_) => Ok(req),
                Err(e) => {
                    tracing::error!(error = %e, "Failed to commit transaction");
                    return Err(CreateLocalNoteError::Unknown(e.into()));
                }
            }
        }

        async fn create_remote_note(
            &self,
            req: RemoteNote,
        ) -> Result<RemoteNote, CreateRemoteNoteError> {
            let client = self.get_client().await?;

            let result = queries::insert_note(
                &client,
                &req.id,
                &req.actor_id,
                None,
                &req.content,
                req.note_url.as_str(),
            )
            .await;

            match result {
                Ok(_) => Ok(req),
                Err(e) => {
                    tracing::error!(error = %e, "Failed to insert remote note");
                    Err(CreateRemoteNoteError::Unknown(e.into()))
                }
            }
        }
    }
}
