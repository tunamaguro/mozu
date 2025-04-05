use std::sync::Arc;

use crate::domain::{account::adapter::AccountService, hosturl::HostUrlService};

pub trait AppRegistryExt: Send + Sync {
    fn account_service(&self) -> Arc<dyn AccountService>;
    fn host_url_service(&self) -> Arc<dyn HostUrlService>;
}

#[derive(Clone)]
pub struct AppRegistry {
    account_service: Arc<dyn AccountService>,
    host_url_service: Arc<dyn HostUrlService>,
}

impl AppRegistry {
    pub fn new<AS, HS>(account_service: AS, host_url_service: HS) -> Self
    where
        AS: AccountService,
        HS: HostUrlService,
    {
        Self {
            account_service: Arc::new(account_service),
            host_url_service: Arc::new(host_url_service),
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
}
