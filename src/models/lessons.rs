use serde::{Deserialize, Serialize};

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
    //rooms: []
}
