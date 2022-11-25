use axum::{
    extract::{Host, Path, Query},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;

static HOST: &str = "https://minipub.loca.lt";
static HOSTNAME: &str = "minipub.loca.lt";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users/:id", get(user_get_handler))
        .route("/.well-known/webfinger", get(webfinger_get_handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct PersonActivity {
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
async fn user_get_handler(Path(user_id): Path<String>, host: Host) -> Json<PersonActivity> {
    info!("user_get: query: {:?}", user_id);
    info!("from: {:?}", host);
    Json(PersonActivity {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        _type: "Person".to_string(),
        id: format!("{}/users/{}", HOST, user_id),
        name: user_id.clone(),
        preferred_username: user_id.clone(),
        summary: "".to_string(),
        inbox: format!("{}/users/{}/inbox", HOST, user_id),
        outbox: format!("{}/users/{}/outbox", HOST, user_id),
        url: format!("{}/users/{}", HOST, user_id),
    })
}

#[derive(Deserialize, Debug)]
struct WebFingerQuery {
    resource: String,
}
#[derive(Serialize)]
struct WebFingerResponse {
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
async fn webfinger_get_handler(query: Query<WebFingerQuery>) -> Json<WebFingerResponse> {
    info!("webfinger_get query: {:?}", query);
    let query_cap = regex::Regex::new(r"^acct:([^@]+)@(.+)$")
        .unwrap()
        .captures(&query.resource)
        .unwrap();
    let user_name = query_cap.get(1).unwrap().as_str();
    let response = WebFingerResponse {
        subject: format!("acct:{}@{}", user_name, HOSTNAME),
        aliases: vec![format!("{}/user/{}", HOST, user_name)],
        links: vec![WebFingerResponseLink {
            rel: Some("self".to_string()),
            _type: "application/activity+json".to_string(),
            href: Some(format!("{HOST}/users/{}", user_name)),
        }],
    };

    Json(response)
}
