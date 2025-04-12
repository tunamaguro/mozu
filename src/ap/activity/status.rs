use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::domain::HttpUrl;

use super::Audience;

/// Status activity types based on Mastodon's implementation
/// https://docs.joinmastodon.org/spec/activitypub/#status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum StatusType {
    Create,
    Update,
    Delete,
    Announce,
    Like,
}

/// ActivityPub Status Activity implementation
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StatusActivity {
    pub id: HttpUrl,
    #[serde(rename = "type")]
    pub kind: StatusType,
    pub actor: HttpUrl,

    #[serde(flatten)]
    #[builder(default)]
    pub audience: Audience,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub object: Option<HttpUrl>,
}