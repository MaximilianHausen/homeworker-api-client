use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::Date;

// Status: Checked but unfinished

#[derive(Serialize, Deserialize, Clone)]
pub struct TimetableDay {
    pub date: Date,
    pub is_today: bool,
    pub is_unschooled: bool,
    pub lessons: Vec<Entry>,
    pub representations: Vec<()>, //TODO: Find representation type
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub unit: Unit,
    pub lessons: Vec<Lesson>,
    pub is_break: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Unit {
    pub id: Option<u32>, //TODO: Check unit id type (propably u32)
    pub start: String,
    pub end: String,
    pub position: Option<u8>,
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
    pub color: Option<String>, //TODO: Check lesson color type (propably String)
    pub import_source: String,
    pub is_gradable: bool,
    pub is_timetable_only: bool,
    pub rooms: HashMap<String, String>,
}
