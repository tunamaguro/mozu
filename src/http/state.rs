use std::sync::Arc;

use crate::{
    Postgres,
    domain::{
        account::adapter::AccountService,
        ap::adapter::ActorService,
        hosturl::{HostUrl, HostUrlService},
    },
};

pub trait AppRegistryExt: Send + Sync {
    fn account_service(&self) -> Arc<dyn AccountService>;
    fn host_url_service(&self) -> Arc<dyn HostUrlService>;
    fn actor_service(&self) -> Arc<dyn ActorService>;
}

#[derive(Clone)]
pub struct AppRegistry {
    account_service: Arc<dyn AccountService>,
    host_url_service: Arc<dyn HostUrlService>,
    actor_service: Arc<dyn ActorService>,
}

impl AppRegistry {
    pub fn from_pg_host_url(pg: Postgres, host_url: HostUrl) -> Self {
        let host_url_service = Arc::new(host_url);

        let webfinger =
            crate::domain::ap::webfinger::WebFingerResolver::new(host_url_service.clone());
        let actor_service = crate::domain::ap::service::Service::new(
            pg.clone(),
            pg.clone(),
            webfinger,
            host_url_service.clone(),
        );

        let account_repository =
            crate::domain::account::service::Service::new(pg.clone(), actor_service.clone());

        Self {
            account_service: Arc::new(account_repository),
            actor_service: Arc::new(actor_service),
            host_url_service,
        }
    }
}

impl AppRegistryExt for AppRegistry {
    fn account_service(&self) -> Arc<dyn AccountService> {
        self.account_service.clone()
    }

    fn host_url_service(&self) -> Arc<dyn HostUrlService> {
        self.host_url_service.clone()
    }

    fn actor_service(&self) -> Arc<dyn ActorService> {
        self.actor_service.clone()
    }
}
