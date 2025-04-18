use std::sync::Arc;

use crate::{
    Postgres,
    domain::{
        account::{self, adapter::AccountService},
        ap::{self, adapter::ApService},
        hosturl::{HostUrl, HostUrlService},
    },
};

pub trait AppRegistryExt: Send + Sync {
    fn account_service(&self) -> Arc<dyn AccountService>;
    fn host_url_service(&self) -> Arc<dyn HostUrlService>;
    fn ap_service(&self) -> Arc<dyn ApService>;
}

#[derive(Clone)]
pub struct AppRegistry {
    account_service: Arc<dyn AccountService>,
    host_url_service: Arc<dyn HostUrlService>,
    ap_service: Arc<dyn ApService>,
}

impl AppRegistry {
    pub fn from_pg_host_url(pg: Postgres, host_url: HostUrl) -> Self {
        let host_url = Arc::new(host_url);

        let ap_service = ap::service::Service::new(pg.clone(), pg.clone(), host_url.clone());

        let account_service = account::service::Service::new(pg.clone(), ap_service.clone());

        Self {
            account_service: Arc::new(account_service),
            ap_service: Arc::new(ap_service),
            host_url_service: host_url,
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

    fn ap_service(&self) -> Arc<dyn ApService> {
        self.ap_service.clone()
    }
}
