pub mod account;
pub mod hosturl;

use std::{ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
