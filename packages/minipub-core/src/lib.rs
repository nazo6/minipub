use axum::{routing::get, Router};
use tracing::info;

mod constant;
mod routing;

#[derive(Debug)]
pub struct Config {
    /// The domain of the server. This domain is used for fediverse response.
    pub domain: String,
    /// The domain name to bind.
    pub bind_domain: Option<String>,
    /// The domain of the web frontend. This domain is used for fediverse response.
    pub ui_domain: Option<String>,
    /// Url of database.
    pub db_url: String,
    /// DB name.
    pub db_name: String,
}

/// Start the server.
///
/// * `config` - Config of the server.
pub async fn start(config: &Config) {
    constant::DOMAIN.set(config.domain.clone()).unwrap();
    constant::BASE_URL
        .set(format!("https://{}", config.domain.clone()))
        .unwrap();

    let bind_domain = match &config.bind_domain {
        Some(bind_domain) => bind_domain.clone(),
        None => config.domain.clone(),
    };

    tracing_subscriber::fmt::init();

    info!(
        "Starting server on: {}, Recognized domain is: {}",
        &bind_domain, &config.domain
    );

    #[rustfmt::skip]
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/:id", get(routing::user::user_get))
        .route("/.well-known/webfinger", get(routing::well_known::webfinger_get));

    axum::Server::bind(&bind_domain.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
