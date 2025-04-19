pub mod account;
pub mod ap;
pub mod hosturl;

use std::{ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<T> {
    inner: uuid::Uuid,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Id<T> {
    pub fn new() -> Self {
        Self {
            inner: uuid::Uuid::now_v7(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self {
            inner: uuid,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Deref for Id<T> {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> AsRef<uuid::Uuid> for Id<T> {
    fn as_ref(&self) -> &uuid::Uuid {
        &self.inner
    }
}

impl<T> From<uuid::Uuid> for Id<T> {
    fn from(uuid: uuid::Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

impl<T> From<Id<T>> for uuid::Uuid {
    fn from(id: Id<T>) -> Self {
        id.inner
    }
}

impl<T> FromStr for Id<T> {
    type Err = <uuid::Uuid as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = uuid::Uuid::from_str(s)?;
        Ok(Id::from_uuid(uuid))
    }
}

impl<T> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let uuid = uuid::Uuid::deserialize(deserializer)?;
        Ok(Id::from_uuid(uuid))
    }
}

/// Wrapper for `url::Url` to ensure it is `http` or `https` and has a host.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HttpUrl(url::Url);

const VALID_SCHEMES: &[&str] = &["http", "https"];

#[derive(Debug, Clone, thiserror::Error)]
pub enum HttpUrlError {
    #[error(transparent)]
    InvalidUrl(#[from] url::ParseError),
    #[error("missing host")]
    MissingHost,
    #[error("expected `http` or `https` scheme")]
    InvalidScheme,
}

impl HttpUrl {
    pub fn new(url: url::Url) -> Result<Self, HttpUrlError> {
        if !VALID_SCHEMES.contains(&url.scheme()) {
            return Err(HttpUrlError::InvalidScheme);
        }

        if url.host_str().is_none() {
            return Err(HttpUrlError::MissingHost);
        }

        Ok(Self(url))
    }

    pub fn host(&self) -> &str {
        self.0.host_str().unwrap()
    }

    pub fn scheme(&self) -> &str {
        self.0.scheme()
    }
}

impl Deref for HttpUrl {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HttpUrl> for url::Url {
    fn from(host_url: HttpUrl) -> Self {
        host_url.0
    }
}

impl TryFrom<url::Url> for HttpUrl {
    type Error = HttpUrlError;

    fn try_from(url: url::Url) -> Result<Self, Self::Error> {
        Self::new(url)
    }
}
impl FromStr for HttpUrl {
    type Err = HttpUrlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = url::Url::parse(s)?;
        Self::new(url)
    }
}
impl std::fmt::Display for HttpUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for HttpUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HttpUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let url = url::Url::deserialize(deserializer)?;
        Self::new(url).map_err(serde::de::Error::custom)
    }
}
