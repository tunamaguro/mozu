pub mod relationship;
pub mod status;

pub use relationship::RelationshipActivity;
pub use status::StatusActivity;

use serde::{Deserialize, Serialize};

use crate::domain::HttpUrl;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Activity {
    Status(StatusActivity),
    Relationship(Box<RelationshipActivity>),
}

/// ActivityPub audience fields
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Audience {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<HttpUrl>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<HttpUrl>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bto: Option<Vec<HttpUrl>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<HttpUrl>>,
}
