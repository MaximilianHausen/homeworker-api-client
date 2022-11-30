use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::{Date, PrimitiveDateTime, Time};
use url::Url;

//TODO: Redo courses/lessons from API responses

/*#[derive(Serialize, Deserialize, Clone)]
pub struct Course {
    id: i32,
    name: String,
    //name_sync: null,
    school_id: i32,
    created: i32,
    timetable_week: i32,
    is_teacher_mode: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Lesson {
    id: i32,
    name: String,
    short: String,
    room: String,
    teacher: String,
    note: String,
    color: String,
    //import_source: null,
    is_gradable: bool,
    is_timetable_only: bool,
    rooms: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileEntry {
    id: i32,
    name: String,
    is_dir: bool,
    parent_id: String,
    created_at: NaiveDateTime,
    document: Option<File>,
    can_write: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    id: i32,
    hash: String,
    sha1_checksum: String,
    name: String,
    content_type: String,
    size: i32,
    related_to: String,
    uploaded: NaiveDateTime,
    url: Url,
    preview_url: Url,
    download_url: Url,
    can_delete: bool,
    human_size: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Homework {
    id: i32,
    course_id: i32,
    lesson_id: i32,
    text: String,
    note: String,
    time: i32,
    until_datetime: NaiveDateTime,
    //related_members: [],
    is_private: bool,
    is_restricted: bool,
    can_edit: bool,
    is_checked: bool,
    can_check: bool,
    is_submissionable: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Timetable {
    date: NaiveDate,
    is_today: bool,
    is_unschooled: bool,
    lessons: Vec<TimetableLesson>,
    //representations: []
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimetableLesson {
    unit: TimetableTime,
    lessons: Option<Vec<Lesson>>,
    is_break: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimetableTime {
    //id: null,
    start: Time,
    end: Time,
    //position: null,
    text: String,
    text_readable: String,
}*/
