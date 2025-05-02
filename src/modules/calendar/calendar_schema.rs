use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;
use crate::modules::calendar::calendar_model::{TimeSlot, BufferTime};

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