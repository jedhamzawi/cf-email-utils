//! Models for listing e-mail routing rules from the [Cloudflare API](https://developers.cloudflare.com/api/operations/email-routing-routing-rules-list-routing-rules)

use serde_derive::{Deserialize, Serialize};

use super::RoutingRule;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ListResponse {
    pub result: Vec<RoutingRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<ListResultInfo>,
    pub errors: Vec<ListResponseError>,
    pub messages: Vec<ListResponseError>,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ListResultInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ListResponseError {
    pub code: usize,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ListResponseMessage {
    pub code: usize,
    pub message: String,
}
