mod app;
mod config;
mod errors;
mod middleware;
mod modules;
mod services;
mod utils;

use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Start the application
    app::create_app().await.map_err(|e| {
        eprintln!("Application error: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })
}
