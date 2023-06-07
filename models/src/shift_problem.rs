use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::note::Note;

#[derive(Serialize, Deserialize)]
pub struct MinimamlShiftProblem {
    pub id: Uuid,
    pub shift_id: Uuid,
    pub maintainer_id: Uuid,
    pub machine_id: Uuid,
    pub begin_time: NaiveTime,
    pub end_time: NaiveTime,
    pub problems_ids: Vec<Uuid>,
    pub spare_parts_ids: Option<Vec<Uuid>>,
    pub note: Option<Note>,
}

#[derive(Serialize, Deserialize)]
pub struct ShiftProblem {
    pub id: Uuid,
    pub shift_id: Uuid,
    pub maintainer_id: Uuid,
    pub machine_id: Uuid,
    pub begin_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Serialize, Deserialize)]
pub struct ProblemDetail {
    pub shift_id: Uuid,
    pub maintainer_id: Uuid,
    pub machine_id: Uuid,
    pub begin_time: NaiveTime,
    pub end_time: NaiveTime,
    pub problems_ids: Vec<Uuid>,
    pub spare_parts_ids: Option<Vec<Uuid>>,
    pub note: Option<String>,
}
