use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub enum ShiftOrder {
    FIRST,
    SECOND,
    THIRD,
}

#[derive(Serialize, Deserialize)]
pub struct Shift {
    pub id: Uuid,
    pub shift_date: NaiveDate,
    pub shift_order: ShiftOrder,
}

#[derive(Serialize, Deserialize)]
pub struct DepartmentShift {
    pub id: Uuid,
    pub shift_id: Uuid,
    pub department_id: Uuid,
}
