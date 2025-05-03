use actix_web::{web, HttpResponse};
use mongodb::Database;
use validator::Validate;
use serde_json::json;
use mongodb::bson::{oid::ObjectId, DateTime};
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Duration, Datelike};

use crate::errors::error::AppError;
use crate::modules::user::user_schema::Claims;
use crate::modules::calendar::calendar_crud::{CalendarSettingsRepository, AvailabilityRepository};
use crate::modules::calendar::calendar_model::{CalendarSettings, Availability, AvailabilityRule, AvailabilitySlot, BufferTime};
use crate::modules::calendar::calendar_schema::{
    CreateCalendarSettingsRequest, CalendarSettingsResponse,
    CreateAvailabilityRequest, CreateAvailabilityRuleRequest,
    AvailabilityResponse, CheckAvailabilityRequest, 
    CheckAvailabilityResponse, AvailableTimeSlot
};

pub struct CalendarController {
    settings_repository: CalendarSettingsRepository,
    availability_repository: AvailabilityRepository,
}

impl CalendarController {
    pub fn new(db: Database) -> Self {
        let settings_repository = CalendarSettingsRepository::new(db.clone());
        let availability_repository = AvailabilityRepository::new(db);
        Self { settings_repository, availability_repository }
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
        let created_settings = self.settings_repository.create(&user_id, settings).await?;

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
        let existing_settings = self.settings_repository.find_by_user_id(&user_id).await?
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
        let updated_settings = self.settings_repository.update(&existing_settings.id.unwrap(), settings).await?
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
        let existing_settings = self.settings_repository.find_by_user_id(&user_id).await?
            .ok_or_else(|| AppError::NotFound("Calendar settings not found".to_string()))?;

        // Delete from database
        self.settings_repository.delete(&existing_settings.id.unwrap()).await?
            .ok_or_else(|| AppError::NotFound("Failed to delete calendar settings".to_string()))?;

        Ok(HttpResponse::Ok().json(json!({
            "message": "Calendar settings deleted successfully"
        })))
    }

    pub async fn create_availability(
        &self,
        claims: web::ReqData<Claims>,
        data: web::Json<CreateAvailabilityRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Validate request data
        data.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let calendar_settings_id = ObjectId::parse_str(&data.calendar_settings_id)
            .map_err(|_| AppError::BadRequest("Invalid calendar settings ID".to_string()))?;

        // Verify calendar settings exist and belong to user
        let settings = self.settings_repository.find_by_user_id(&user_id).await?
            .ok_or_else(|| AppError::NotFound("Calendar settings not found".to_string()))?;

        if settings.id.unwrap() != calendar_settings_id {
            return Err(AppError::BadRequest("Calendar settings do not belong to user".to_string()));
        }

        // Convert rules using the new method
        let mut processed_rules = Vec::new();
        for rule in &data.rules {
            let processed_rule = AvailabilityRule::new(
                &rule.start_date,
                rule.end_date.as_deref(),
                rule.is_recurring,
                rule.recurrence_pattern.clone(),
                rule.slots.clone(),
            ).map_err(|e| AppError::ValidationError(e))?;
            processed_rules.push(processed_rule);
        }

        // Create new availability
        let availability = Availability {
            id: None,
            user_id,
            calendar_settings_id,
            rules: processed_rules,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        // Save to database
        let created = self.availability_repository.create(availability).await?;

        // Convert to response
        let response = AvailabilityResponse {
            id: created.id.unwrap().to_hex(),
            user_id: created.user_id.to_hex(),
            calendar_settings_id: created.calendar_settings_id.to_hex(),
            rules: created.rules,
            created_at: created.created_at.to_string(),
            updated_at: created.updated_at.to_string(),
        };

        Ok(HttpResponse::Created().json(response))
    }

    pub async fn check_availability(
        &self,
        claims: web::ReqData<Claims>,
        data: web::Json<CheckAvailabilityRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Validate request data
        data.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        // Get calendar settings for buffer times
        let settings = self.settings_repository.find_by_user_id(&user_id).await?
            .ok_or_else(|| AppError::NotFound("Calendar settings not found".to_string()))?;

        // Parse dates
        let start_date = DateTime::parse_rfc3339_str(&data.start_date)
            .map_err(|_| AppError::BadRequest("Invalid start date format".to_string()))?;
        let end_date = DateTime::parse_rfc3339_str(&data.end_date)
            .map_err(|_| AppError::BadRequest("Invalid end date format".to_string()))?;

        // Get user's availability
        let availabilities = self.availability_repository
            .find_available_slots(&user_id, start_date, end_date)
            .await?;

        // Process available slots
        let mut available_slots = Vec::new();
        for availability in availabilities {
            for rule in availability.rules {
                if let Some(mut slots) = self.process_availability_rule(
                    rule, 
                    &start_date, 
                    &end_date, 
                    data.duration,
                    &settings.buffer_time
                ) {
                    available_slots.append(&mut slots);
                }
            }
        }

        // Sort slots by date and start time
        available_slots.sort_by(|a, b| {
            a.date.cmp(&b.date).then(a.start_time.cmp(&b.start_time))
        });

        Ok(HttpResponse::Ok().json(CheckAvailabilityResponse {
            available_slots,
        }))
    }

    fn process_availability_rule(
        &self,
        rule: AvailabilityRule,
        start_date: &DateTime,
        end_date: &DateTime,
        duration: i32,
        buffer_time: &BufferTime,
    ) -> Option<Vec<AvailableTimeSlot>> {
        let mut available_slots = Vec::new();
        
        // Convert BSON DateTime to NaiveDate
        let start_naive = NaiveDateTime::from_timestamp_millis(start_date.timestamp_millis())
            .map(|dt| dt.date())
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
            
        let end_naive = NaiveDateTime::from_timestamp_millis(end_date.timestamp_millis())
            .map(|dt| dt.date())
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());

        let mut current_date = start_naive;

        while current_date <= end_naive {
            let day_of_week = match current_date.weekday() {
                chrono::Weekday::Mon => "monday",
                chrono::Weekday::Tue => "tuesday",
                chrono::Weekday::Wed => "wednesday",
                chrono::Weekday::Thu => "thursday",
                chrono::Weekday::Fri => "friday",
                chrono::Weekday::Sat => "saturday",
                chrono::Weekday::Sun => "sunday",
            };
            
            // Find matching slots for the current day
            for slot in rule.slots.iter() {
                if slot.day_of_week != day_of_week || !slot.is_available {
                    continue;
                }

                // Parse slot times
                let slot_start = NaiveTime::parse_from_str(&slot.start_time, "%H:%M")
                    .unwrap_or_else(|_| NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                let slot_end = NaiveTime::parse_from_str(&slot.end_time, "%H:%M")
                    .unwrap_or_else(|_| NaiveTime::from_hms_opt(23, 59, 59).unwrap());

                // Calculate available time slots considering duration and buffer times
                let mut current_time = slot_start;
                let total_duration = duration + buffer_time.before + buffer_time.after;

                while current_time + Duration::minutes(total_duration as i64) <= slot_end {
                    // Add buffer before
                    let actual_start = current_time + Duration::minutes(buffer_time.before as i64);
                    let actual_end = actual_start + Duration::minutes(duration as i64);

                    available_slots.push(AvailableTimeSlot {
                        date: current_date.format("%Y-%m-%d").to_string(),
                        start_time: actual_start.format("%H:%M").to_string(),
                        end_time: actual_end.format("%H:%M").to_string(),
                    });

                    // Move to next slot including buffer after
                    current_time = actual_end + Duration::minutes(buffer_time.after as i64);
                }
            }

            // Move to next day
            current_date = current_date.succ_opt().unwrap_or(end_naive);
        }

        Some(available_slots)
    }
}