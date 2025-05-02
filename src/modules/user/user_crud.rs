use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use crate::modules::user::user_model::User;

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new() -> Self {
        let db = crate::app::AppState::get().db.clone();
        Self {
            collection: db.collection("users"),
        }
    }

    pub async fn create(&self, user: User) -> Result<User, mongodb::error::Error> {
        let mut user = user;
        let result = self.collection.insert_one(&user, None).await?;
        user.id = result.inserted_id.as_object_id();
        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.collection
            .find_one(doc! { "email": email }, None)
            .await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, mongodb::error::Error> {
        let object_id = match ObjectId::parse_str(id) {
            Ok(id) => id,
            Err(_) => return Ok(None),
        };
        
        self.collection
            .find_one(doc! { "_id": object_id }, None)
            .await
    }

    pub async fn find_by_verification_token(&self, token: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.collection
            .find_one(doc! { "verification_token": token }, None)
            .await
    }

    pub async fn find_by_refresh_token(&self, token: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.collection
            .find_one(doc! { "refresh_token": token }, None)
            .await
    }

    pub async fn find_by_password_reset_token(&self, token: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.collection
            .find_one(doc! { "password_reset_token": token }, None)
            .await
    }

    pub async fn update(&self, id: &str, user: &User) -> Result<Option<User>, mongodb::error::Error> {
        let object_id = match ObjectId::parse_str(id) {
            Ok(id) => id,
            Err(_) => return Ok(None),
        };

        self.collection
            .find_one_and_replace(doc! { "_id": object_id }, user, None)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), mongodb::error::Error> {
        let object_id = match ObjectId::parse_str(id) {
            Ok(id) => id,
            Err(_) => return Ok(()),
        };

        self.collection
            .delete_one(doc! { "_id": object_id }, None)
            .await?;
        
        Ok(())
    }
}
