use std::{str::FromStr, sync::LazyLock};

use crate::domain::HttpUrl;

use super::constants::{ACTIVITYPUB_MEDIA_TYPE, ACTIVITYPUB_MEDIA_TYPE_ALT};
use serde::{Deserialize, Serialize};

/// See https://datatracker.ietf.org/doc/html/rfc7033
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebFinger {
    pub subject: String,
    #[serde(default)]
    pub links: Vec<WebFingerLink>,
}

impl WebFinger {
    pub fn actor_link(&self) -> Option<&HttpUrl> {
        self.links.iter().find_map(|link| link.actor_link())
    }
}

/// See https://datatracker.ietf.org/doc/html/rfc7033#section-4.4.4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebFingerLink {
    pub rel: String,
    /// `type` is a reserved keyword in Rust, so we use `kind` instead
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub href: Option<HttpUrl>,
}

impl WebFingerLink {
    /// Check if the link resolves to actor
    ///
    /// See https://docs.joinmastodon.org/spec/webfinger/#mastodons-requirements-for-webfinger
    pub fn is_self_link(&self) -> bool {
        let is_self = self.rel == "self";
        let Some(kind) = self.kind.as_ref() else {
            return false;
        };
        let is_activitypub = kind == ACTIVITYPUB_MEDIA_TYPE || kind == ACTIVITYPUB_MEDIA_TYPE_ALT;
        is_self && is_activitypub
    }

    /// Return href if the link resolves to actor
    pub fn actor_link(&self) -> Option<&HttpUrl> {
        if self.is_self_link() {
            self.href.as_ref()
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
