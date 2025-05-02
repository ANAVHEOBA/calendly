use actix_web::{web, HttpResponse};
use mongodb::Database;
use validator::Validate;
use serde_json::json;

use crate::errors::error::AppError;
use crate::modules::user::user_schema::Claims;
use crate::modules::calendar::calendar_crud::CalendarSettingsRepository;
use crate::modules::calendar::calendar_model::CalendarSettings;
use crate::modules::calendar::calendar_schema::{CreateCalendarSettingsRequest, CalendarSettingsResponse};
use mongodb::bson::{oid::ObjectId, DateTime};

pub struct CalendarController {
    repository: CalendarSettingsRepository,
}

impl CalendarController {
    pub fn new(db: Database) -> Self {
        let repository = CalendarSettingsRepository::new(db);
        Self { repository }
    }

    pub async fn create_settings(
        &self,
        claims: web::ReqData<Claims>,
        data: web::Json<CreateCalendarSettingsRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Validate request data
        data.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        // Create new calendar settings
        let settings = CalendarSettings {
            id: None,
            user_id,
            timezone: data.timezone.clone(),
            working_hours: data.working_hours.clone(),
            buffer_time: data.buffer_time.clone(),
            default_meeting_duration: data.default_meeting_duration,
            calendar_name: data.calendar_name.clone(),
            date_format: data.date_format.clone(),
            time_format: data.time_format.clone(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        // Save to database
        let created_settings = self.repository.create(&user_id, settings).await?;

        // Convert to response
        let response = CalendarSettingsResponse {
            id: created_settings.id.unwrap().to_hex(),
            user_id: created_settings.user_id.to_hex(),
            timezone: created_settings.timezone,
            working_hours: created_settings.working_hours,
            buffer_time: created_settings.buffer_time,
            default_meeting_duration: created_settings.default_meeting_duration,
            calendar_name: created_settings.calendar_name,
            date_format: created_settings.date_format,
            time_format: created_settings.time_format,
            created_at: created_settings.created_at.to_string(),
            updated_at: created_settings.updated_at.to_string(),
        };

        Ok(HttpResponse::Created().json(response))
    }

    pub async fn update_settings(
        &self,
        claims: web::ReqData<Claims>,
        data: web::Json<CreateCalendarSettingsRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Validate request data
        data.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        // Find existing settings
        let existing_settings = self.repository.find_by_user_id(&user_id).await?
            .ok_or_else(|| AppError::NotFound("Calendar settings not found".to_string()))?;

        // Create updated settings
        let settings = CalendarSettings {
            id: existing_settings.id,
            user_id,
            timezone: data.timezone.clone(),
            working_hours: data.working_hours.clone(),
            buffer_time: data.buffer_time.clone(),
            default_meeting_duration: data.default_meeting_duration,
            calendar_name: data.calendar_name.clone(),
            date_format: data.date_format.clone(),
            time_format: data.time_format.clone(),
            created_at: existing_settings.created_at,
            updated_at: DateTime::now(),
        };

        // Update in database
        let updated_settings = self.repository.update(&existing_settings.id.unwrap(), settings).await?
            .ok_or_else(|| AppError::NotFound("Failed to update calendar settings".to_string()))?;

        // Convert to response
        let response = CalendarSettingsResponse {
            id: updated_settings.id.unwrap().to_hex(),
            user_id: updated_settings.user_id.to_hex(),
            timezone: updated_settings.timezone,
            working_hours: updated_settings.working_hours,
            buffer_time: updated_settings.buffer_time,
            default_meeting_duration: updated_settings.default_meeting_duration,
            calendar_name: updated_settings.calendar_name,
            date_format: updated_settings.date_format,
            time_format: updated_settings.time_format,
            created_at: updated_settings.created_at.to_string(),
            updated_at: updated_settings.updated_at.to_string(),
        };

        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn delete_settings(
        &self,
        claims: web::ReqData<Claims>,
    ) -> Result<HttpResponse, AppError> {
        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        // Find existing settings
        let existing_settings = self.repository.find_by_user_id(&user_id).await?
            .ok_or_else(|| AppError::NotFound("Calendar settings not found".to_string()))?;

        // Delete from database
        self.repository.delete(&existing_settings.id.unwrap()).await?
            .ok_or_else(|| AppError::NotFound("Failed to delete calendar settings".to_string()))?;

        Ok(HttpResponse::Ok().json(json!({
            "message": "Calendar settings deleted successfully"
        })))
    }
}