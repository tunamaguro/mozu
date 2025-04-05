use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// See https://www.w3.org/TR/activitystreams-vocabulary/#actor-types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorType {
    Person,
    Application,
    Service,
    Group,
    Organization,
}

/// See https://www.w3.org/TR/activitystreams-core/#actors
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Actor {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: ActorType,
    pub inbox: String,
    pub outbox: String,

    /// used for user displayed name
    ///
    /// See https://docs.joinmastodon.org/spec/activitypub/#properties-used-1
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,

    /// used for webfinger lookup.
    /// So this is used for mention like `@preferredUsername@domain`.
    ///
    /// See https://docs.joinmastodon.org/spec/activitypub/#properties-used-1
    #[serde(rename = "preferredUsername")]
    #[builder(setter(into))]
    pub preferred_username: String,
}
