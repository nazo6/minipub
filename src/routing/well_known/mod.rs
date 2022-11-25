use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::constant::{BASE_URL, DOMAIN};

#[derive(Deserialize, Debug)]
pub(crate) struct WebFingerQuery {
    resource: String,
}
#[derive(Serialize)]
pub(crate) struct WebFingerResponse {
    subject: String,
    aliases: Vec<String>,
    links: Vec<WebFingerResponseLink>,
}
#[derive(Serialize)]
struct WebFingerResponseLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    rel: Option<String>,
    #[serde(rename = "type")]
    _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
}
pub(crate) async fn webfinger_get(query: Query<WebFingerQuery>) -> Json<WebFingerResponse> {
    info!("webfinger_get query: {:?}", query);
    let query_cap = regex::Regex::new(r"^acct:([^@]+)@(.+)$").unwrap().captures(&query.resource).unwrap();
    let user_name = query_cap.get(1).unwrap().as_str();
    let response = WebFingerResponse {
        subject: format!("acct:{}@{}", user_name, DOMAIN),
        aliases: vec![format!("{}/user/{}", BASE_URL, user_name)],
        links: vec![WebFingerResponseLink {
            rel: Some("self".to_string()),
            _type: "application/activity+json".to_string(),
            href: Some(format!("{BASE_URL}/users/{}", user_name)),
        }],
    };

    Json(response)
}
