use std::sync::LazyLock;

use super::constants::{ACTIVITYPUB_MEDIA_TYPE, ACTIVITYPUB_MEDIA_TYPE_ALT};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// See https://datatracker.ietf.org/doc/html/rfc7033
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct WebFinger {
    subject: String,
    #[serde(default)]
    links: Vec<WebFingerLink>,
}

/// See https://datatracker.ietf.org/doc/html/rfc7033#section-4.4.4
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct WebFingerLink {
    rel: String,
    /// `type` is a reserved keyword in Rust, so we use `kind` instead
    #[serde(rename = "type")]
    kind: String,
    href: String,
}

impl WebFingerLink {
    /// Check if the link resolves to actor
    ///
    /// See https://docs.joinmastodon.org/spec/webfinger/#mastodons-requirements-for-webfinger
    pub fn is_self_link(&self) -> bool {
        let is_self = self.rel == "self";
        let is_activitypub =
            self.kind == ACTIVITYPUB_MEDIA_TYPE || self.kind == ACTIVITYPUB_MEDIA_TYPE_ALT;
        is_self && is_activitypub
    }

    /// Return href if the link resolves to actor
    pub fn actor_link(&self) -> Option<&str> {
        if self.is_self_link() {
            Some(&self.href)
        } else {
            None
        }
    }
}

/// WebFinger media types
///
/// See https://datatracker.ietf.org/doc/html/rfc7033#section-10.2
pub const WEBFINGER_MEDIA_TYPE: &str = "application/jrd+json";

/// WebFinger MIME
pub static WEBFINGER_MIME: LazyLock<mime::Mime> =
    LazyLock::new(|| WEBFINGER_MEDIA_TYPE.parse().unwrap());
