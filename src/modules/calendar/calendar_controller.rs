use actix_web::{web, HttpResponse};
use mongodb::Database;
use validator::Validate;
use serde_json::json;
use mongodb::bson::{oid::ObjectId, DateTime};
use chrono::{NaiveTime, Duration};

use crate::errors::error::AppError;
use crate::modules::user::user_schema::Claims;
use crate::modules::calendar::calendar_crud::{CalendarSettingsRepository, AvailabilityRepository, EventTypeRepository};
use crate::modules::calendar::calendar_model::{CalendarSettings, Availability, AvailabilityRule, EventType, BufferTime};
use crate::modules::calendar::calendar_schema::{
    CreateCalendarSettingsRequest, CalendarSettingsResponse,
    CreateAvailabilityRequest, AvailabilityResponse, CheckAvailabilityRequest, 
    CheckAvailabilityResponse, AvailableTimeSlot,
    CreateEventTypeRequest, EventTypeResponse, CheckTimeSlotRequest, CheckTimeSlotResponse,
    UpdateAvailabilityRequest, UpdateEventTypeRequest
};

pub struct CalendarController {
    settings_repository: CalendarSettingsRepository,
    availability_repository: AvailabilityRepository,
    event_type_repository: EventTypeRepository,
}

impl CalendarController {
    pub fn new(db: Database) -> Self {
        let settings_repository = CalendarSettingsRepository::new(db.clone());
        let availability_repository = AvailabilityRepository::new(db.clone());
        let event_type_repository = EventTypeRepository::new(db);
        Self { 
            settings_repository, 
            availability_repository,
            event_type_repository 
        }
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
        let start_date = chrono::DateTime::from_timestamp_millis(start_date.timestamp_millis())
            .map(|dt| dt.date_naive())
            .unwrap_or_default();
        let end_date = chrono::DateTime::from_timestamp_millis(end_date.timestamp_millis())
            .map(|dt| dt.date_naive())
            .unwrap_or_default();
        let mut current_date = start_date;

        while current_date <= end_date {
            let day_of_week = current_date.format("%A").to_string().to_lowercase();
            
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
            current_date = current_date.succ_opt().unwrap_or(end_date);
        }

        Some(available_slots)
    }

    pub async fn create_event_type(
        &self,
        claims: web::ReqData<Claims>,
        data: web::Json<CreateEventTypeRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Validate request data
        data.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        // Validate location type
        let valid_location_types = vec!["in_person", "phone", "video"];
        if !valid_location_types.contains(&data.location_type.as_str()) {
            return Err(AppError::BadRequest("Invalid location type".to_string()));
        }

        // Validate meeting link for video type
        if data.location_type == "video" && data.meeting_link.is_none() {
            return Err(AppError::BadRequest("Meeting link is required for video events".to_string()));
        }

        // Validate color format
        if !data.color.starts_with('#') || data.color.len() != 7 {
            return Err(AppError::BadRequest("Invalid color format. Use hex color code (e.g., #FF0000)".to_string()));
        }

        // Validate availability schedule exists and belongs to user
        let availability_id = ObjectId::parse_str(&data.availability_schedule_id)
            .map_err(|_| AppError::BadRequest("Invalid availability schedule ID".to_string()))?;

        let availability = self.availability_repository.find_by_id(&availability_id).await?
            .ok_or_else(|| AppError::NotFound("Availability schedule not found".to_string()))?;

        if availability.user_id != user_id {
            return Err(AppError::Forbidden("Availability schedule does not belong to user".to_string()));
        }

        // Create new event type
        let event_type = EventType {
            id: None,
            user_id,
            name: data.name.clone(),
            description: data.description.clone(),
            duration: data.duration,
            color: data.color.clone(),
            location_type: data.location_type.clone(),
            meeting_link: data.meeting_link.clone(),
            questions: data.questions.clone(),
            availability_schedule_id: availability_id,
            buffer_time: data.buffer_time.clone(),
            min_booking_notice: data.min_booking_notice,
            max_booking_notice: data.max_booking_notice,
            is_active: data.is_active,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        // Save to database
        let created = self.event_type_repository.create(event_type).await?;

        // Convert to response
        let response = EventTypeResponse {
            id: created.id.unwrap().to_hex(),
            user_id: created.user_id.to_hex(),
            name: created.name,
            description: created.description,
            duration: created.duration,
            color: created.color,
            location_type: created.location_type,
            meeting_link: created.meeting_link,
            questions: created.questions,
            availability_schedule_id: created.availability_schedule_id.to_hex(),
            buffer_time: created.buffer_time,
            min_booking_notice: created.min_booking_notice,
            max_booking_notice: created.max_booking_notice,
            is_active: created.is_active,
            created_at: created.created_at.to_string(),
            updated_at: created.updated_at.to_string(),
        };

        Ok(HttpResponse::Created().json(response))
    }

    pub async fn get_settings(
        &self,
        claims: web::ReqData<Claims>,
    ) -> Result<HttpResponse, AppError> {
        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let settings = self.settings_repository.find_by_user_id(&user_id).await?
            .ok_or_else(|| AppError::NotFound("Calendar settings not found".to_string()))?;

        let response = CalendarSettingsResponse {
            id: settings.id.unwrap().to_hex(),
            user_id: settings.user_id.to_hex(),
            timezone: settings.timezone,
            working_hours: settings.working_hours,
            buffer_time: settings.buffer_time,
            default_meeting_duration: settings.default_meeting_duration,
            calendar_name: settings.calendar_name,
            date_format: settings.date_format,
            time_format: settings.time_format,
            created_at: settings.created_at.to_string(),
            updated_at: settings.updated_at.to_string(),
        };

        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn check_time_slot(
        &self,
        claims: web::ReqData<Claims>,
        data: web::Json<CheckTimeSlotRequest>,
    ) -> Result<HttpResponse, AppError> {
        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        // Get calendar settings
        let settings = self.settings_repository.find_by_user_id(&user_id).await?
            .ok_or_else(|| AppError::NotFound("Calendar settings not found".to_string()))?;

        // Get user's availability
        let availability = self.availability_repository.find_by_user_id(&user_id).await?
            .ok_or_else(|| AppError::NotFound("Availability not found".to_string()))?;

        // Check if the time slot is available
        let mut conflicts = Vec::new();
        let is_available = self.is_slot_available(
            &data.date,
            &data.start_time,
            &data.end_time,
            &settings,
            &availability,
            &mut conflicts,
        );

        Ok(HttpResponse::Ok().json(CheckTimeSlotResponse {
            is_available,
            conflicts: if conflicts.is_empty() { None } else { Some(conflicts) },
        }))
    }

    pub async fn update_availability(
        &self,
        claims: web::ReqData<Claims>,
        id: web::Path<String>,
        data: web::Json<UpdateAvailabilityRequest>,
    ) -> Result<HttpResponse, AppError> {
        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let availability_id = ObjectId::parse_str(&*id)
            .map_err(|_| AppError::BadRequest("Invalid availability ID".to_string()))?;

        // Check if availability exists and belongs to user
        let existing = self.availability_repository.find_by_id(&availability_id).await?
            .ok_or_else(|| AppError::NotFound("Availability not found".to_string()))?;

        if existing.user_id != user_id {
            return Err(AppError::Forbidden("Availability does not belong to user".to_string()));
        }

        // Process rules
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

        // Update availability
        let mut updated = existing;
        updated.rules = processed_rules;
        updated.updated_at = DateTime::now();

        let result = self.availability_repository.update(&availability_id, updated).await?
            .ok_or_else(|| AppError::NotFound("Failed to update availability".to_string()))?;

        let response = AvailabilityResponse {
            id: result.id.unwrap().to_hex(),
            user_id: result.user_id.to_hex(),
            calendar_settings_id: result.calendar_settings_id.to_hex(),
            rules: result.rules,
            created_at: result.created_at.to_string(),
            updated_at: result.updated_at.to_string(),
        };

        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn delete_availability(
        &self,
        claims: web::ReqData<Claims>,
        id: web::Path<String>,
    ) -> Result<HttpResponse, AppError> {
        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let availability_id = ObjectId::parse_str(&*id)
            .map_err(|_| AppError::BadRequest("Invalid availability ID".to_string()))?;

        // Check if availability exists and belongs to user
        let existing = self.availability_repository.find_by_id(&availability_id).await?
            .ok_or_else(|| AppError::NotFound("Availability not found".to_string()))?;

        if existing.user_id != user_id {
            return Err(AppError::Forbidden("Availability does not belong to user".to_string()));
        }

        // Delete availability
        self.availability_repository.delete(&availability_id).await?
            .ok_or_else(|| AppError::NotFound("Failed to delete availability".to_string()))?;

        Ok(HttpResponse::Ok().json(json!({
            "message": "Availability deleted successfully"
        })))
    }

    fn is_slot_available(
        &self,
        date: &str,
        start_time: &str,
        end_time: &str,
        settings: &CalendarSettings,
        availability: &Availability,
        conflicts: &mut Vec<String>,
    ) -> bool {
        // Check if date is within working hours
        let day_of_week = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .ok()
            .and_then(|d| Some(d.format("%A").to_string().to_lowercase()))
            .unwrap_or_default();

        if let Some(working_hours) = settings.working_hours.get(&day_of_week) {
            if working_hours.is_empty() {
                conflicts.push("No working hours set for this day".to_string());
                return false;
            }

            // Check if time slot is within working hours
            let slot_start = NaiveTime::parse_from_str(start_time, "%H:%M")
                .unwrap_or_else(|_| NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            let slot_end = NaiveTime::parse_from_str(end_time, "%H:%M")
                .unwrap_or_else(|_| NaiveTime::from_hms_opt(23, 59, 59).unwrap());

            let is_within_working_hours = working_hours.iter().any(|wh| {
                let wh_start = NaiveTime::parse_from_str(&wh.start, "%H:%M")
                    .unwrap_or_else(|_| NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                let wh_end = NaiveTime::parse_from_str(&wh.end, "%H:%M")
                    .unwrap_or_else(|_| NaiveTime::from_hms_opt(23, 59, 59).unwrap());
                slot_start >= wh_start && slot_end <= wh_end
            });

            if !is_within_working_hours {
                conflicts.push("Time slot is outside working hours".to_string());
                return false;
            }
        } else {
            conflicts.push("No working hours set for this day".to_string());
            return false;
        }

        // Check if time slot is within availability rules
        let is_within_availability = availability.rules.iter().any(|rule| {
            self.is_slot_available_in_rule(rule, date, start_time, end_time)
        });

        if !is_within_availability {
            conflicts.push("Time slot is not available in your schedule".to_string());
            return false;
        }

        true
    }

    fn is_slot_available_in_rule(
        &self,
        rule: &AvailabilityRule,
        date: &str,
        start_time: &str,
        end_time: &str,
    ) -> bool {
        // Check if date is within rule's date range
        let slot_date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .ok()
            .and_then(|d| Some(d.and_hms_opt(0, 0, 0).unwrap()))
            .unwrap_or_default();

        let rule_start = chrono::DateTime::from_timestamp_millis(rule.start_date.timestamp_millis())
            .map(|dt| dt.date_naive().and_hms_opt(0, 0, 0).unwrap())
            .unwrap_or_default();
        let rule_end = rule.end_date
            .map(|d| chrono::DateTime::from_timestamp_millis(d.timestamp_millis())
                .map(|dt| dt.date_naive().and_hms_opt(0, 0, 0).unwrap())
                .unwrap_or_default())
            .unwrap_or_else(|| chrono::NaiveDateTime::MAX);

        if slot_date < rule_start || slot_date > rule_end {
            return false;
        }

        // Check if time slot matches any availability slot
        let day_of_week = slot_date.format("%A").to_string().to_lowercase();
        let slot_start = NaiveTime::parse_from_str(start_time, "%H:%M")
            .unwrap_or_else(|_| NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let slot_end = NaiveTime::parse_from_str(end_time, "%H:%M")
            .unwrap_or_else(|_| NaiveTime::from_hms_opt(23, 59, 59).unwrap());

        rule.slots.iter().any(|slot| {
            slot.day_of_week == day_of_week &&
            slot.is_available &&
            NaiveTime::parse_from_str(&slot.start_time, "%H:%M")
                .map(|s| s <= slot_start)
                .unwrap_or(false) &&
            NaiveTime::parse_from_str(&slot.end_time, "%H:%M")
                .map(|e| e >= slot_end)
                .unwrap_or(false)
        })
    }

    pub async fn list_event_types(
        &self,
        claims: web::ReqData<Claims>,
    ) -> Result<HttpResponse, AppError> {
        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let event_types = self.event_type_repository.find_by_user_id(&user_id).await?;

        let response: Vec<EventTypeResponse> = event_types.into_iter().map(|et| EventTypeResponse {
            id: et.id.unwrap().to_hex(),
            user_id: et.user_id.to_hex(),
            name: et.name,
            description: et.description,
            duration: et.duration,
            color: et.color,
            location_type: et.location_type,
            meeting_link: et.meeting_link,
            questions: et.questions,
            availability_schedule_id: et.availability_schedule_id.to_hex(),
            buffer_time: et.buffer_time,
            min_booking_notice: et.min_booking_notice,
            max_booking_notice: et.max_booking_notice,
            is_active: et.is_active,
            created_at: et.created_at.to_string(),
            updated_at: et.updated_at.to_string(),
        }).collect();

        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn update_event_type(
        &self,
        claims: web::ReqData<Claims>,
        id: web::Path<String>,
        data: web::Json<UpdateEventTypeRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Validate request data
        data.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let event_type_id = ObjectId::parse_str(&*id)
            .map_err(|_| AppError::BadRequest("Invalid event type ID".to_string()))?;

        // Check if event type exists and belongs to user
        let existing = self.event_type_repository.find_by_id(&event_type_id).await?
            .ok_or_else(|| AppError::NotFound("Event type not found".to_string()))?;

        if existing.user_id != user_id {
            return Err(AppError::Forbidden("Event type does not belong to user".to_string()));
        }

        // Validate location type if provided
        if let Some(location_type) = &data.location_type {
            let valid_location_types = vec!["in_person", "phone", "video"];
            if !valid_location_types.contains(&location_type.as_str()) {
                return Err(AppError::BadRequest("Invalid location type".to_string()));
            }

            // Validate meeting link for video type
            if location_type == "video" && data.meeting_link.is_none() {
                return Err(AppError::BadRequest("Meeting link is required for video events".to_string()));
            }
        }

        // Validate color format if provided
        if let Some(color) = &data.color {
            if !color.starts_with('#') || color.len() != 7 {
                return Err(AppError::BadRequest("Invalid color format. Use hex color code (e.g., #FF0000)".to_string()));
            }
        }

        // Update event type
        let mut updated = existing;
        if let Some(name) = &data.name { updated.name = name.clone(); }
        if let Some(description) = &data.description { updated.description = Some(description.clone()); }
        if let Some(duration) = data.duration { updated.duration = duration; }
        if let Some(color) = &data.color { updated.color = color.clone(); }
        if let Some(location_type) = &data.location_type { updated.location_type = location_type.clone(); }
        if let Some(meeting_link) = &data.meeting_link { updated.meeting_link = Some(meeting_link.clone()); }
        if let Some(questions) = &data.questions { updated.questions = questions.clone(); }
        if let Some(buffer_time) = &data.buffer_time { updated.buffer_time = Some(buffer_time.clone()); }
        if let Some(min_booking_notice) = data.min_booking_notice { updated.min_booking_notice = Some(min_booking_notice); }
        if let Some(max_booking_notice) = data.max_booking_notice { updated.max_booking_notice = Some(max_booking_notice); }
        if let Some(is_active) = data.is_active { updated.is_active = is_active; }
        updated.updated_at = DateTime::now();

        let result = self.event_type_repository.update(&event_type_id, updated).await?
            .ok_or_else(|| AppError::NotFound("Failed to update event type".to_string()))?;

        let response = EventTypeResponse {
            id: result.id.unwrap().to_hex(),
            user_id: result.user_id.to_hex(),
            name: result.name,
            description: result.description,
            duration: result.duration,
            color: result.color,
            location_type: result.location_type,
            meeting_link: result.meeting_link,
            questions: result.questions,
            availability_schedule_id: result.availability_schedule_id.to_hex(),
            buffer_time: result.buffer_time,
            min_booking_notice: result.min_booking_notice,
            max_booking_notice: result.max_booking_notice,
            is_active: result.is_active,
            created_at: result.created_at.to_string(),
            updated_at: result.updated_at.to_string(),
        };

        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn delete_event_type(
        &self,
        claims: web::ReqData<Claims>,
        id: web::Path<String>,
    ) -> Result<HttpResponse, AppError> {
        let claims = claims.into_inner();
        let user_id = ObjectId::parse_str(&claims.sub)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let event_type_id = ObjectId::parse_str(&*id)
            .map_err(|_| AppError::BadRequest("Invalid event type ID".to_string()))?;

        // Check if event type exists and belongs to user
        let existing = self.event_type_repository.find_by_id(&event_type_id).await?
            .ok_or_else(|| AppError::NotFound("Event type not found".to_string()))?;

        if existing.user_id != user_id {
            return Err(AppError::Forbidden("Event type does not belong to user".to_string()));
        }

        // Delete event type
        self.event_type_repository.delete(&event_type_id).await?
            .ok_or_else(|| AppError::NotFound("Failed to delete event type".to_string()))?;

        Ok(HttpResponse::Ok().json(json!({
            "message": "Event type deleted successfully"
        })))
    }
}