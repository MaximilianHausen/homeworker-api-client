use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use super::lessons;

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    id: i32,
    //course_id: null,
    lesson_id: i32,
    text: String,
    note: String,
    points: i32,
    time: i32,
    until_datetime: PrimitiveDateTime,
    //import_source: null,
    //import_key: null,
    created_at: PrimitiveDateTime,
    posted_on: PrimitiveDateTime,
    to_be_done_until: PrimitiveDateTime,
    lesson: lessons::Lesson,
    note_formatted: String,
    note_basic: String,
    is_checked: bool,
    can_check: bool,
    is_submissionable: bool,
    document_relation: String,
    can_correct: bool,
    can_edit: bool,
    is_admin: bool,
    can_delete: bool,
    //documents: []
}
