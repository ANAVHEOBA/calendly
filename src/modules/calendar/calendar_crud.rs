use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Collection, Database,
};
use futures::TryStreamExt;
use crate::errors::error::AppError;
use crate::modules::calendar::calendar_model::{CalendarSettings, Availability};


pub struct CalendarSettingsRepository {
    collection: Collection<CalendarSettings>,
}

impl CalendarSettingsRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection("calendar_settings");
        Self { collection }
    }

    pub async fn create(&self, user_id: &ObjectId, settings: CalendarSettings) -> Result<CalendarSettings, AppError> {
        // Check if settings already exist for user
        if let Ok(Some(_)) = self.find_by_user_id(user_id).await {
            return Err(AppError::BadRequest("Calendar settings already exist for this user".to_string()));
        }

        let mut settings = settings;
        settings.created_at = DateTime::now();
        settings.updated_at = DateTime::now();

        let result = self.collection
            .insert_one(&settings, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        settings.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(settings)
    }

    pub async fn find_by_user_id(&self, user_id: &ObjectId) -> Result<Option<CalendarSettings>, AppError> {
        self.collection
            .find_one(doc! { "user_id": user_id }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn update(&self, id: &ObjectId, settings: CalendarSettings) -> Result<Option<CalendarSettings>, AppError> {
        let mut settings = settings;
        settings.updated_at = DateTime::now();

        let result = self.collection
            .find_one_and_replace(
                doc! { "_id": id },
                &settings,
                None
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    pub async fn delete(&self, id: &ObjectId) -> Result<Option<CalendarSettings>, AppError> {
        self.collection
            .find_one_and_delete(doc! { "_id": id }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}





pub struct AvailabilityRepository {
    collection: Collection<Availability>,
}

impl AvailabilityRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection("availability");
        Self { collection }
    }

    pub async fn create(&self, availability: Availability) -> Result<Availability, AppError> {
        let mut availability = availability;
        availability.created_at = DateTime::now();
        availability.updated_at = DateTime::now();

        let result = self.collection
            .insert_one(&availability, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        availability.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(availability)
    }

    pub async fn find_by_user_id(&self, user_id: &ObjectId) -> Result<Option<Availability>, AppError> {
        self.collection
            .find_one(doc! { "user_id": user_id }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_by_calendar_settings_id(&self, calendar_settings_id: &ObjectId) -> Result<Option<Availability>, AppError> {
        self.collection
            .find_one(doc! { "calendar_settings_id": calendar_settings_id }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn update(&self, id: &ObjectId, availability: Availability) -> Result<Option<Availability>, AppError> {
        let mut availability = availability;
        availability.updated_at = DateTime::now();

        let result = self.collection
            .find_one_and_replace(
                doc! { "_id": id },
                &availability,
                None
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    pub async fn delete(&self, id: &ObjectId) -> Result<Option<Availability>, AppError> {
        self.collection
            .find_one_and_delete(doc! { "_id": id }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_available_slots(&self, user_id: &ObjectId, start_date: DateTime, end_date: DateTime) -> Result<Vec<Availability>, AppError> {
        let filter = doc! {
            "user_id": user_id,
            "$or": [
                {
                    "rules.start_date": { "$lte": end_date },
                    "rules.end_date": { "$gte": start_date }
                },
                {
                    "rules.start_date": { "$lte": end_date },
                    "rules.end_date": null,
                    "rules.is_recurring": true
                }
            ]
        };

        let mut availabilities = Vec::new();
        let mut cursor = self.collection
            .find(filter, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        while let Some(availability) = cursor.try_next().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))? {
            availabilities.push(availability);
        }

        Ok(availabilities)
    }
}