use mongodb::bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub name: String,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub refresh_token: Option<String>,
    pub password_reset_token: Option<String>,
    pub password_reset_expires: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl User {
    pub fn new(email: String, password: String, name: String) -> Self {
        Self {
            id: None,
            email,
            password,
            name,
            is_verified: false,
            verification_token: None,
            refresh_token: None,
            password_reset_token: None,
            password_reset_expires: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub fn set_verification_token(&mut self, token: String) {
        self.verification_token = Some(token);
        self.updated_at = DateTime::now();
    }

    pub fn verify(&mut self) {
        self.is_verified = true;
        self.verification_token = None;
        self.updated_at = DateTime::now();
    }

    pub fn set_refresh_token(&mut self, token: String) {
        self.refresh_token = Some(token);
        self.updated_at = DateTime::now();
    }

    pub fn set_password_reset_token(&mut self, token: String) {
        self.password_reset_token = Some(token);
        let now = Utc::now();
        let expires = now + chrono::Duration::hours(1);
        self.password_reset_expires = Some(DateTime::from_millis(expires.timestamp_millis()));
        self.updated_at = DateTime::now();
    }

    pub fn clear_password_reset_token(&mut self) {
        self.password_reset_token = None;
        self.password_reset_expires = None;
        self.updated_at = DateTime::now();
    }
}
