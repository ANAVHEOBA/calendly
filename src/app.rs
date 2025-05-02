use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use mongodb::{Client, Database};
use crate::config::environment::Environment;
use crate::modules::user::user_router::user_routes;
use crate::errors::error::AppError;
use std::sync::OnceLock;

static APP_STATE: OnceLock<AppState> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Database,
}

impl AppState {
    pub fn get() -> &'static AppState {
        APP_STATE.get().expect("AppState not initialized")
    }
}

pub async fn create_app() -> Result<(), AppError> {
    // Load environment variables
    dotenv::dotenv().ok();
    let env = Environment::load();
    
    // Initialize database
    let client = Client::with_uri_str(&env.mongodb_uri)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to connect to MongoDB: {}", e)))?;
    
    // Get database instance
    let db = client.database(&env.database_name);
    
    // Verify database connection
    db.run_command(mongodb::bson::doc! { "ping": 1 }, None)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to ping database: {}", e)))?;
    
    // Initialize global AppState
    APP_STATE.set(AppState { db: db.clone() }).expect("Failed to set AppState");
    
    let app_state = web::Data::new(AppState { db });

    // Create HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .configure(|cfg| {
                        if let Ok(routes) = user_routes() {
                            cfg.service(routes);
                        }
                    })
            )
    })
    .bind(("127.0.0.1", env.port))?
    .run()
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))
}
