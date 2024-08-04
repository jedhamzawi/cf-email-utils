//! Contains models based off of the [Cloudflare API](https://developers.cloudflare.com/api/)

use serde_derive::{Deserialize, Serialize};

pub(crate) mod delete;
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
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RouteAction {
    pub r#type: RouteActionType,
    pub value: Vec<String>,
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
    pub field: RouteMatcherField,
    pub r#type: RouteMatcherType,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RouteMatcherField {
    To,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RouteMatcherType {
    Literal,
}

// "result": [

//   {

//     "actions": [

//       {

//         "type": "forward",

//         "value": [

//           "destinationaddress@example.net"

//         ]

//       }

//     ],

//     "enabled": true,

//     "id": "a7e6fb77503c41d8a7f3113c6918f10c",

//     "matchers": [

//       {

//         "field": "to",

//         "type": "literal",

//         "value": "test@example.com"

//       }

//     ],

//     "name": "Send to user@example.net rule.",

//     "priority": 0,

//     "tag": "a7e6fb77503c41d8a7f3113c6918f10c"

//   }

// ],
