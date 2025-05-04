use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeSlot {
    pub start: String,  // Format: "HH:mm"
    pub end: String,    // Format: "HH:mm"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BufferTime {
    pub before: i32,  // minutes
    pub after: i32,   // minutes
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarSettings {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub timezone: String,
    pub working_hours: HashMap<String, Vec<TimeSlot>>,
    pub buffer_time: BufferTime,
    pub default_meeting_duration: i32,
    pub calendar_name: String,
    pub date_format: String,
    pub time_format: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvailabilitySlot {
    pub day_of_week: String,  // "monday", "tuesday", etc.
    pub start_time: String,   // Format: "HH:mm"
    pub end_time: String,     // Format: "HH:mm"
    pub is_available: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvailabilityRule {
    pub start_date: DateTime,
    pub end_date: Option<DateTime>,
    pub is_recurring: bool,
    pub recurrence_pattern: Option<String>,  // "daily", "weekly", "monthly"
    pub slots: Vec<AvailabilitySlot>,
}

impl AvailabilityRule {
    pub fn new(start_date_str: &str, end_date_str: Option<&str>, is_recurring: bool, recurrence_pattern: Option<String>, slots: Vec<AvailabilitySlot>) -> Result<Self, String> {
        let start_date = DateTime::parse_rfc3339_str(start_date_str)
            .map_err(|e| format!("Invalid start date: {}", e))?;
        
        let end_date = if let Some(date_str) = end_date_str {
            Some(DateTime::parse_rfc3339_str(date_str)
                .map_err(|e| format!("Invalid end date: {}", e))?)
        } else {
            None
        };

        Ok(Self {
            start_date,
            end_date,
            is_recurring,
            recurrence_pattern,
            slots,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Availability {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub calendar_settings_id: ObjectId,
    pub rules: Vec<AvailabilityRule>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventType {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub name: String,
    pub description: Option<String>,
    pub duration: i32,
    pub color: String,
    pub location_type: String,
    pub meeting_link: Option<String>,
    pub questions: Vec<String>,
    pub availability_schedule_id: ObjectId,
    pub buffer_time: Option<BufferTime>,
    pub min_booking_notice: Option<i32>,
    pub max_booking_notice: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
} 
 