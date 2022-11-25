use minipub_core::Config;

#[tokio::main]
async fn main() {
    let config: Config = Config {
        domain: env!("DOMAIN").to_string(),
        bind_domain: Some(format!("{}:{}", env!("LOCAL_HOST"), env!("PORT"))),
        ui_domain: None,
        db_url: env!("DB_URL").to_string(),
        db_name: env!("DB_NAME").to_string(),
    };
    dbg!(&config);
    minipub_core::start(&config).await;
}
