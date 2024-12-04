mod app;
mod routes;
mod config;
mod logger;
mod schema;
mod models;

#[tokio::main]
async fn main() {
    let file_config = std::env::args().nth(1).expect("No file configuration provided");
    let config = config::config::read_config(&file_config);

    let mut log = logger::AppLogger::new(config.logger);
    log.info("Starting application".to_string());

    let action = std::env::args().nth(2).expect("No action provided");
    if action.to_lowercase() == "setup" || action.to_lowercase() == "init" {
        if !std::path::Path::new(".env").exists() {
            std::fs::write(".env", format!("DATABASE_URL={}", config.postgre_url)).unwrap();
            log.info("Environment file created".to_string());
        } else {
            log.warn("Environment file already exists".to_string());
        }

        std::process::exit(0);
    }

    log.info("Starting REST Server".to_string());
    let app = app::create_app().await;
    
    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", config.listen_host, config.listen_port)
    ).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
