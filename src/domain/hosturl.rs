use std::sync::Arc;

use url::Url;

use super::HttpUrl;

#[derive(Debug, Clone)]
pub struct HostUrl {
    scheme: String,
    host: String,
}

impl HostUrl {
    pub fn new(host_url: &str) -> Self {
        let url = Url::parse(host_url).expect("Invalid URL");
        let host = url.host_str().expect("missing host_str").to_string();
        let scheme = url.scheme().to_string();
        Self { scheme, host }
    }
}

impl HostUrlService for HostUrl {
    fn scheme(&self) -> &str {
        &self.scheme
    }

    fn host(&self) -> &str {
        &self.host
    }
}

pub trait HostUrlService: Send + Sync + 'static {
    fn scheme(&self) -> &str;
    fn host(&self) -> &str;

    /// Returns the base URL `scheme://host`
    fn base_url(&self) -> HttpUrl {
        format!("{}://{}", self.scheme(), self.host())
            .parse()
            .unwrap()
    }

    /// Return actor URL
    fn actor_url(&self, user: &str) -> HttpUrl {
        format!("{}/ap/actors/{}", self.base_url(), user)
            .parse()
            .unwrap()
    }

    /// Return shared inbox URL
    fn shared_inbox_url(&self) -> HttpUrl {
        format!("{}/ap/inbox", self.base_url()).parse().unwrap()
    }

    /// Return actor_name inbox URL
    fn inbox_url(&self, actor_name: &str) -> HttpUrl {
        format!("{}/inbox", self.actor_url(actor_name))
            .parse()
            .unwrap()
    }

    /// Return actor_name outbox URL
    fn outbox_url(&self, actor_name: &str) -> HttpUrl {
        format!("{}/outbox", self.actor_url(actor_name))
            .parse()
            .unwrap()
    }

    /// Return user note URL
    fn note_url(&self, note_id: &str) -> HttpUrl {
        format!("{}/ap/notes/{}", self.base_url(), note_id)
            .parse()
            .unwrap()
    }
}

impl<S: HostUrlService> HostUrlService for Arc<S> {
    fn scheme(&self) -> &str {
        self.as_ref().scheme()
    }

    fn host(&self) -> &str {
        self.as_ref().host()
    }
}
