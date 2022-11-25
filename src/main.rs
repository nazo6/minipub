use axum::{routing::get, Router};
use const_format::formatcp;

mod constant;
mod routing;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    #[rustfmt::skip]
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/:id", get(routing::user::user_get))
        .route("/.well-known/webfinger", get(routing::well_known::webfinger_get));

    #[cfg(debug_assertions)]
    axum::Server::bind(&formatcp!("{}:{}", env!("LOCAL_HOST"), env!("PORT")).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    #[cfg(not(debug_assertions))]
    axum::Server::bind(&formatcp!("{}:{}", env!("DOMAIN"), env!("PORT")).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
