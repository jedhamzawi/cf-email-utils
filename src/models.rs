//! Contains models based off of the [Cloudflare API](https://developers.cloudflare.com/api/)

use serde_derive::{Deserialize, Serialize};

pub(crate) mod list;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RoutingRule {
    pub actions: Vec<RouteAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub matchers: Vec<RouteMatcher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RouteAction {
    pub r#type: RouteActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RouteActionType {
    Drop,
    Forward,
    Worker,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RouteMatcher {
    pub r#type: RouteMatcherType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<RouteMatcherField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RouteMatcherField {
    To,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RouteMatcherType {
    Literal,
    All,
}
