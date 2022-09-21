use std::collections::HashMap;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// Status: Checked but unfinished

#[derive(Serialize, Deserialize, Clone)]
pub struct TimetableDay {
    pub date: NaiveDate,
    pub is_today: bool,
    pub is_unschooled: bool,
    pub lessons: Vec<Entry>,
    pub representations: Vec<Representation>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub unit: Unit,
    pub lessons: Vec<Lesson>,
    pub is_break: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Unit {
    pub id: Option<u32>,
    //TODO: Check unit id type (propably u32)
    pub start: String,
    pub end: String,
    pub position: Option<String>,
    //TODO: Find unit position type (placeholder String)
    pub positions: Vec<u8>,
    pub text: String,
    pub text_readable: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Lesson {
    pub id: u32,
    pub name: String,
    pub short: String,
    pub room: String,
    pub teacher: String,
    pub note: String,
    pub color: Option<String>,
    //TODO: Check lesson color type (propably String)
    pub import_source: String,
    pub is_gradable: bool,
    pub is_timetable_only: bool,
    pub rooms: HashMap<String, String>,
}
