//! ActivityPub module
use mime;
mod actor;
mod webfinger;
use std::sync::LazyLock;

pub use actor::{Actor, ActorType};
pub use webfinger::{WebFinger, WebFingerLink};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context<T> {
    /// The JSON-LD context for the object.
    ///
    /// See https://www.w3.org/TR/activitystreams-core/#jsonld
    #[serde(rename = "@context")]
    pub context: serde_json::Value,
    pub inner: T,
}

impl<T> Context<T> {
    pub fn new(inner: T) -> Self {
        Self {
            context: serde_json::json!([
                "https://www.w3.org/ns/activitystreams",
                // "https://w3id.org/security/v1",
            ]),
            inner,
        }
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
