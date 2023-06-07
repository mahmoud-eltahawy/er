use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use models::shift::{ShiftOrder,Shift,DepartmentShift};

use super::{
    note::{Note, ShiftNote},
    TableResponse, Wrapable, Stringify, FromExt,
};

const FIRST_SHIFT: &str = "FIRST";
const SECOND_SHIFT: &str = "SECOND";
const THIRD_SHIFT: &str = "THIRD";


impl Stringify for ShiftOrder {
    fn stringify(&self) -> String {
        match self {
            ShiftOrder::FIRST => FIRST_SHIFT.to_string(),
            ShiftOrder::SECOND => SECOND_SHIFT.to_string(),
            ShiftOrder::THIRD => THIRD_SHIFT.to_string(),
        }
    }
}

impl FromExt for ShiftOrder {
    fn ext_from(value: String) -> Result<Self, String> {
        match value.as_str() {
            FIRST_SHIFT => Ok(ShiftOrder::FIRST),
            SECOND_SHIFT => Ok(ShiftOrder::SECOND),
            THIRD_SHIFT => Ok(ShiftOrder::THIRD),
            _ => Err("undefined shift".to_string()),
        }
    }
}

impl Wrapable for DepartmentShift {
    fn wrap(self) -> TableResponse {
        TableResponse::DepartmentShift(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateDepartmentShift {
    SaveShiftEmployee(Uuid, Uuid),
    DeleteShiftEmployee(Uuid, Uuid),
    SaveNote(ShiftNote),
    DeleteNote(Uuid, Uuid),
    UpdateNote(Note),
}
