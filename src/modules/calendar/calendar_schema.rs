use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;
use mongodb::bson::DateTime;
use crate::modules::calendar::calendar_model::{TimeSlot, BufferTime, AvailabilityRule, AvailabilitySlot};

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