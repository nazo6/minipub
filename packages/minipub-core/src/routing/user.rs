use axum::{
    extract::{Host, Path},
    Json,
};
use serde::Serialize;
use tracing::info;

use crate::constant::BASE_URL;

#[derive(Serialize)]
pub(crate) struct PersonActivity {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "type")]
    _type: String,
    id: String,
    name: String,
    #[serde(rename = "preferredUsername")]
    preferred_username: String,
    summary: String,
    inbox: String,
    outbox: String,
    url: String,
}
pub(crate) async fn user_get(Path(user_id): Path<String>, host: Host) -> Json<PersonActivity> {
    info!("user_get: query: {:?}", user_id);
    info!("from: {:?}", host);
    let base_url = BASE_URL.get().unwrap();
    Json(PersonActivity {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        _type: "Person".to_string(),
        id: format!("{}/user/{}", base_url, user_id),
        name: user_id.clone(),
        preferred_username: user_id.clone(),
        summary: "".to_string(),
        inbox: format!("{}/user/{}/inbox", base_url, user_id),
        outbox: format!("{}/user/{}/outbox", base_url, user_id),
        url: format!("{}/user/{}", base_url, user_id),
    })
}
