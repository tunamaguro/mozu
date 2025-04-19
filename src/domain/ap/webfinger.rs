use super::{adapter::WebFingerPort, model::actor::WebFingerError};
use crate::ap;
use crate::domain::{HttpUrl, hosturl::HostUrlService};
use anyhow::Context;
use reqwest::header;

pub struct WebFingerResolver<HS> {
    host_url: HS,
    client: reqwest::Client,
}

impl<HS> WebFingerResolver<HS>
where
    HS: HostUrlService,
{
    pub fn new(hs: HS) -> Self {
        const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(USER_AGENT),
        );
        let host_header =
            header::HeaderValue::from_str(hs.host()).expect("Failed to create header value");
        headers.insert(header::HOST, host_header);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Failed to create reqwest client");

        Self {
            client,
            host_url: hs,
        }
    }
}

impl<HS> WebFingerPort for WebFingerResolver<HS>
where
    HS: HostUrlService,
{
    #[tracing::instrument(skip(self))]
    async fn lookup_by_id(&self, actor_id: &HttpUrl) -> Result<crate::ap::Actor, WebFingerError> {
        let request = self
            .client
            .get(actor_id.as_str())
            .header(header::ACCEPT, ap::constants::ACTIVITYPUB_MEDIA_TYPE);

        let response = request.send().await.context("Cannot send request")?;
        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            return Err(WebFingerError::HttpStatusError(status.as_u16()));
        }

        let actor = response
            .json::<crate::ap::Actor>()
            .await
            .map_err(|e| WebFingerError::UnexpectedResponse(e.to_string()))?;

        if &actor.id != actor_id {
            return Err(WebFingerError::UnexpectedResponse(
                "actor id is not matched".to_string(),
            ));
        };

        Ok(actor)
    }

    #[tracing::instrument(skip(self))]
    async fn lookup_by_host_name(
        &self,
        host: &str,
        name: &str,
    ) -> Result<crate::ap::Actor, WebFingerError> {
        let webfinger_url = format!(
            "https://{}/.well-known/webfinger?resource=acct:{}@{}",
            host, name, host
        );

        let webfinger_resp = self
            .client
            .get(webfinger_url)
            .header(header::ACCEPT, ap::constants::WEBFINGER_MEDIA_TYPE)
            .send()
            .await
            .context("Cannot send request")?;

        let status = webfinger_resp.status();
        if status.is_client_error() || status.is_server_error() {
            return Err(WebFingerError::HttpStatusError(status.as_u16()));
        }

        let webfinger = webfinger_resp
            .json::<ap::WebFinger>()
            .await
            .map_err(|e| WebFingerError::UnexpectedResponse(e.to_string()))?;

        let Some(rel) = webfinger.actor_link() else {
            return Err(WebFingerError::MissingSelfLink);
        };

        self.lookup_by_id(rel).await
    }
}
