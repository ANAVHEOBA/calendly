use mongodb::{Client, Database};
use std::sync::OnceLock;

static DB: OnceLock<Database> = OnceLock::new();

pub async fn init_database(mongodb_uri: &str, database_name: &str) -> Result<(), mongodb::error::Error> {
    let client = Client::with_uri_str(mongodb_uri).await?;
    let database = client.database(database_name);
    
    // Store the database instance in the static variable
    DB.set(database).expect("Failed to set database instance");
    
    Ok(())
}

pub fn get_database() -> &'static Database {
    DB.get().expect("Database not initialized")
}
