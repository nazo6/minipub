use minipub_core::Config;

#[tokio::main]
async fn main() {
    let config: Config = Config {
        domain: env!("DOMAIN").to_string(),
        bind_domain: Some(format!("{}:{}", env!("LOCAL_HOST"), env!("PORT"))),
        ui_domain: None,
        db_url: "sqlite://".to_string(),
    };
    dbg!(&config);
    minipub_core::start(&config).await;
}
