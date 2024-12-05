use logger::AppLogger;
use state::CONFIG_STATES;

mod app;
mod routes;
mod config;
mod logger;
mod schema;
mod models;
mod database;
mod state;
mod services;
mod middlewares;
mod dto;
mod entities;

#[tokio::main]
async fn main() {
    let mut logger = AppLogger::new(&CONFIG_STATES.logger);
    logger.info("Starting application".to_string());

    let action = std::env::args().nth(1).expect("No action provided");
    if action.to_lowercase() == "setup" || action.to_lowercase() == "init" {
        if !std::path::Path::new(".env").exists() {
            std::fs::write(".env", format!("DATABASE_URL={}", CONFIG_STATES.postgre_url)).unwrap();
            logger.info("Environment file created".to_string());
        } else {
            logger.warn("Environment file already exists".to_string());
        }

        std::process::exit(0);
    }

    logger.info("Starting REST Server".to_string());
    let app = app::create_app().await;
    
    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", CONFIG_STATES.listen_host, CONFIG_STATES.listen_port)
    ).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
