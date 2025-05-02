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