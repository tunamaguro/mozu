use url::Url;

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
    fn base_url(&self) -> String {
        format!("{}://{}", self.scheme(), self.host())
    }

    /// Return user URL
    fn user_url(&self, user: &str) -> String {
        format!("{}/{}", self.base_url(), user)
    }
}
