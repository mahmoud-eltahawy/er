use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{note::Note, TableResponse, Wrapable};
pub use models::shift_problem::{MinimamlShiftProblem,ShiftProblem};

impl Wrapable for ShiftProblem {
    fn wrap(self) -> TableResponse {
        TableResponse::ShiftProblem(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateShiftProblem {
    AddProblem(Uuid, Uuid),
    DeleteProblem(Uuid, Uuid),
    AddSparePart(Uuid, Uuid),
    DeleteSparePart(Uuid, Uuid),
    UpdateMaintainer(Uuid, Uuid),
    UpdateMachine(Uuid, Uuid),
    UpdateBeginTime(Uuid, NaiveTime),
    UpdateEndTime(Uuid, NaiveTime),
    AddNote(Note),
    DeleteNote(Uuid),
    UpdateNote(Note),
}

pub trait Msp {
    fn new(problem: ProblemDetail) -> Self;
    fn destruct(self) -> (ShiftProblem, Vec<Uuid>, Option<Vec<Uuid>>, Option<Note>);
}

impl Msp for MinimamlShiftProblem {
    fn new(problem: ProblemDetail) -> Self {
        let ProblemDetail {
            shift_id,
            begin_time,
            end_time,
            machine_id,
            problems_ids,
            spare_parts_ids,
            maintainer_id,
            note,
        } = problem;
        let id = Uuid::new_v4();
        let note = note.map(|content| Note { id, content });
        MinimamlShiftProblem {
            id,
            shift_id,
            maintainer_id,
            machine_id,
            begin_time,
            end_time,
            problems_ids,
            spare_parts_ids,
            note,
        }
    }
    fn destruct(self) -> (ShiftProblem, Vec<Uuid>, Option<Vec<Uuid>>, Option<Note>) {
        let MinimamlShiftProblem {
            id,
            shift_id,
            maintainer_id,
            machine_id,
            begin_time,
            end_time,
            problems_ids,
            spare_parts_ids,
            note,
        } = self;
        let shift_problem = ShiftProblem {
            id,
            machine_id,
            maintainer_id,
            shift_id,
            begin_time,
            end_time,
        };
        (shift_problem, problems_ids, spare_parts_ids, note)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
