use std::{str::FromStr, sync::LazyLock};

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

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ParseAcctUriError {
    #[error("expected `acct:` scheme")]
    InvalidScheme,
    #[error("missing `@`")]
    MissingAt,
    #[error("invalid user")]
    MissingUser,
    #[error("invalid host")]
    MissingHost,
}

/// `acct` Uri
///
/// See https://datatracker.ietf.org/doc/html/rfc7565
#[derive(Debug, Clone)]
pub struct AcctUri {
    pub user: String,
    pub host: String,
}

impl FromStr for AcctUri {
    type Err = ParseAcctUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("acct:")
            .ok_or(ParseAcctUriError::InvalidScheme)?;
        let s = s.strip_prefix("@").unwrap_or(s);

        let (user, host) = s.split_once('@').ok_or(ParseAcctUriError::MissingAt)?;
        if user.is_empty() {
            return Err(ParseAcctUriError::MissingUser);
        }
        if host.is_empty() {
            return Err(ParseAcctUriError::MissingHost);
        }
        Ok(Self {
            user: user.to_string(),
            host: host.to_string(),
        })
    }
}

impl std::fmt::Display for AcctUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "acct:{}@{}", self.user, self.host)
    }
}

impl Serialize for AcctUri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for AcctUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        AcctUri::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// WebFinger media types
///
/// See https://datatracker.ietf.org/doc/html/rfc7033#section-10.2
pub const WEBFINGER_MEDIA_TYPE: &str = "application/jrd+json";

/// WebFinger MIME
pub static WEBFINGER_MIME: LazyLock<mime::Mime> =
    LazyLock::new(|| WEBFINGER_MEDIA_TYPE.parse().unwrap());
