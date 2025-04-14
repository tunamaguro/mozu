//! ActivityPub module
use mime;
use std::sync::LazyLock;

mod activity;
mod actor;
pub mod webfinger;

pub use activity::{Activity, RelationshipActivity, StatusActivity};
pub use actor::{Actor, ActorType};
pub use webfinger::{WebFinger, WebFingerLink};

use serde::{Deserialize, Serialize};

use crate::domain::HttpUrl;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UrlOrObject {
    Url(HttpUrl),
    Object(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context<T> {
    /// The JSON-LD context for the object.
    ///
    /// See https://www.w3.org/TR/activitystreams-core/#jsonld
    #[serde(rename = "@context")]
    pub context: serde_json::Value,
    #[serde(flatten)]
    pub inner: T,
}

impl<T> Context<T> {
    /// Create object with default context
    pub fn new(inner: T) -> Self {
        Self {
            context: serde_json::json!([
                "https://www.w3.org/ns/activitystreams",
                // "https://w3id.org/security/v1",
            ]),
            inner,
        }
    }

    pub fn with_context(context: serde_json::Value, inner: T) -> Self {
        Self { context, inner }
    }

    /// split context and inner
    pub fn split(self) -> (serde_json::Value, T) {
        (self.context, self.inner)
    }
}

pub mod constants {
    use super::*;
    /// ActivityPub media types
    ///
    /// See https://www.w3.org/TR/activitystreams-core/#media-type
    pub const ACTIVITYPUB_MEDIA_TYPE: &str = "application/activity+json";

    /// ActivityPub media types with JSON-LD profile
    ///
    /// See https://www.w3.org/TR/activitystreams-core/#media-type
    pub const ACTIVITYPUB_MEDIA_TYPE_ALT: &str =
        r#"application/ld+json; profile="https://www.w3.org/ns/activitystreams""#;

    /// ActivityPub MIME type
    pub static ACTIVITYPUB_MIME: LazyLock<mime::Mime> =
        LazyLock::new(|| ACTIVITYPUB_MEDIA_TYPE.parse().unwrap());

    /// ActivityPub MIME type with JSON-LD profile
    pub static ACTIVITYPUB_MIME_ALT: LazyLock<mime::Mime> =
        LazyLock::new(|| ACTIVITYPUB_MEDIA_TYPE_ALT.parse().unwrap());

    pub use super::webfinger::{WEBFINGER_MEDIA_TYPE, WEBFINGER_MIME};
}
