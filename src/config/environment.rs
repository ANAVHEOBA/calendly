use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct Environment {
    pub mongodb_uri: String,
    pub database_name: String,
    pub port: u16,
    pub jwt_secret: String,
    pub email_user: String,
    pub email_password: String,
}

impl Environment {
    pub fn load() -> Self {
        println!("Starting Environment::load()");
        
        // Check if .env file exists and can be loaded
        match dotenv() {
            Ok(path) => println!("Loaded .env from: {:?}", path),
            Err(e) => println!("Error loading .env: {:?}", e),
        }

        // Debug: Print all environment variables
        println!("\nAll environment variables:");
        for (key, value) in env::vars() {
            if !key.contains("SECRET") && !key.contains("PASSWORD") {
                println!("{}: {}", key, value);
            } else {
                println!("{}: [HIDDEN]", key);
            }
        }

        // Try to read JWT_SECRET specifically
        println!("\nTrying to read JWT_SECRET:");
        match env::var("JWT_SECRET") {
            Ok(val) => println!("JWT_SECRET found with length: {}", val.len()),
            Err(e) => println!("Error reading JWT_SECRET: {:?}", e),
        }

        // Now try to load all required variables
        println!("\nLoading required variables:");
        let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        println!("✓ MONGODB_URI loaded");
        
        let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        println!("✓ DATABASE_NAME loaded");
        
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a number");
        println!("✓ PORT loaded");
        
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        println!("✓ JWT_SECRET loaded");
        
        let email_user = env::var("EMAIL_USER").expect("EMAIL_USER must be set");
        println!("✓ EMAIL_USER loaded");
        
        let email_password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set");
        println!("✓ EMAIL_PASSWORD loaded");

        Self {
            mongodb_uri,
            database_name,
            port,
            jwt_secret,
            email_user,
            email_password,
        }
    }

    pub fn get_jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
}
