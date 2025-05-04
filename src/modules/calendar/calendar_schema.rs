use std::collections::HashMap;use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::modules::calendar::calendar_model::{
    AvailabilityRule, BufferTime, TimeSlot, AvailabilitySlot
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCalendarSettingsRequest {
    #[validate(length(min = 1, message = "Timezone is required"))]
    pub timezone: String,
    pub working_hours: HashMap<String, Vec<TimeSlot>>,
    pub buffer_time: BufferTime,
    #[validate(range(min = 15, max = 120, message = "Meeting duration must be between 15 and 120 minutes"))]
    pub default_meeting_duration: i32,
    #[validate(length(min = 1, message = "Calendar name is required"))]
    pub calendar_name: String,
    #[validate(length(min = 1, message = "Date format is required"))]
    pub date_format: String,
    #[validate(length(min = 1, message = "Time format is required"))]
    pub time_format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarSettingsResponse {
    pub id: String,
    pub user_id: String,
    pub timezone: String,
    pub working_hours: HashMap<String, Vec<TimeSlot>>,
    pub buffer_time: BufferTime,
    pub default_meeting_duration: i32,
    pub calendar_name: String,
    pub date_format: String,
    pub time_format: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateAvailabilityRuleRequest {
    pub start_date: String,  // ISO 8601 format
    pub end_date: Option<String>,  // ISO 8601 format
    pub is_recurring: bool,
    pub recurrence_pattern: Option<String>,
    pub slots: Vec<AvailabilitySlot>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateAvailabilityRequest {
    pub calendar_settings_id: String,
    #[validate(length(min = 1, message = "At least one availability rule is required"))]
    pub rules: Vec<CreateAvailabilityRuleRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailabilityResponse {
    pub id: String,
    pub user_id: String,
    pub calendar_settings_id: String,
    pub rules: Vec<AvailabilityRule>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CheckAvailabilityRequest {
    pub start_date: String,  // ISO 8601 format
    pub end_date: String,    // ISO 8601 format
    pub duration: i32,       // minutes
}

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AvailableTimeSlot {
    pub date: String,        // YYYY-MM-DD format
    pub start_time: String,  // HH:mm format
    pub end_time: String,    // HH:mm format
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckAvailabilityResponse {
    pub available_slots: Vec<AvailableTimeSlot>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateAvailabilityRequest {
    #[validate(length(min = 1, message = "At least one availability rule is required"))]
    pub rules: Vec<CreateAvailabilityRuleRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CheckTimeSlotRequest {
    pub date: String,         // YYYY-MM-DD format
    pub start_time: String,   // HH:mm format
    pub end_time: String,     // HH:mm format
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckTimeSlotResponse {
    pub is_available: bool,
    pub conflicts: Option<Vec<String>>,  // Reasons why the slot is not available, if any
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateEventTypeRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub description: Option<String>,
    #[validate(range(min = 15, max = 480, message = "Duration must be between 15 and 480 minutes"))]
    pub duration: i32,
    #[validate(length(min = 1, message = "Color is required"))]
    pub color: String,
    #[validate(length(min = 1, message = "Location type is required"))]
    pub location_type: String,
    pub meeting_link: Option<String>,
    pub questions: Vec<String>,
    #[validate(length(min = 1, message = "Availability schedule ID is required"))]
    pub availability_schedule_id: String,
    pub buffer_time: Option<BufferTime>,
    pub min_booking_notice: Option<i32>,
    pub max_booking_notice: Option<i32>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventTypeResponse {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub duration: i32,
    pub color: String,
    pub location_type: String,
    pub meeting_link: Option<String>,
    pub questions: Vec<String>,
    pub availability_schedule_id: String,
    pub buffer_time: Option<BufferTime>,
    pub min_booking_notice: Option<i32>,
    pub max_booking_notice: Option<i32>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateEventTypeRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: Option<String>,
    pub description: Option<String>,
    #[validate(range(min = 15, max = 480, message = "Duration must be between 15 and 480 minutes"))]
    pub duration: Option<i32>,
    #[validate(length(min = 1, message = "Color is required"))]
    pub color: Option<String>,
    #[validate(length(min = 1, message = "Location type is required"))]
    pub location_type: Option<String>,
    pub meeting_link: Option<String>,
    pub questions: Option<Vec<String>>,
    pub buffer_time: Option<BufferTime>,
    pub min_booking_notice: Option<i32>,
    pub max_booking_notice: Option<i32>,
    pub is_active: Option<bool>,
}


