use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::Update;

const EMPLOYEE: &str = "EMPLOYEE";
const MACHINE: &str = "MACHINE";
const DEPARTMENT: &str = "DEPARTMENT";
const SPARE_PART: &str = "SPARE_PART";
const PROBLEM: &str = "PROBLEM";
const SHIFT: &str = "SHIFT";
const DEPARTMENT_SHIFT: &str = "DEPARTMENT_SHIFT";
const SHIFT_PROBLEM: &str = "SHIFT_PROBLEM";

const CREATE: &str = "CREATE";
const DELETE: &str = "DELETE";

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Table {
    Employee,
    Machine,
    Department,
    SparePart,
    Problem,
    Shift,
    ShiftProblem,
    DepartmentShift,
}

impl Table {
    pub fn stringify(&self) -> String {
        match self {
            Table::Employee => EMPLOYEE.to_string(),
            Table::Machine => MACHINE.to_string(),
            Table::Department => DEPARTMENT.to_string(),
            Table::SparePart => SPARE_PART.to_string(),
            Table::Problem => PROBLEM.to_string(),
            Table::Shift => SHIFT.to_string(),
            Table::ShiftProblem => SHIFT_PROBLEM.to_string(),
            Table::DepartmentShift => DEPARTMENT_SHIFT.to_string(),
        }
    }
}
impl TryFrom<String> for Table {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            EMPLOYEE => Ok(Table::Employee),
            MACHINE => Ok(Table::Machine),
            DEPARTMENT => Ok(Table::Department),
            SPARE_PART => Ok(Table::SparePart),
            PROBLEM => Ok(Table::Problem),
            SHIFT => Ok(Table::Shift),
            SHIFT_PROBLEM => Ok(Table::ShiftProblem),
            DEPARTMENT_SHIFT => Ok(Table::DepartmentShift),
            _ => Err("Undefinded table".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Cd {
    Create,
    Delete,
}

impl Cd {
    pub fn stringify(&self) -> Option<String> {
        match self {
            Cd::Create => Some(CREATE.to_string()),
            Cd::Delete => Some(DELETE.to_string()),
        }
    }
}

impl TryFrom<String> for Cd {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            CREATE => Ok(Cd::Create),
            DELETE => Ok(Cd::Delete),
            _ => Err("undefined sql operation".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct CdVersion {
    pub version_number: u64,
    pub updater_id: Uuid,
    pub time_stamp: NaiveDateTime,
    pub target_id: Uuid,
    pub target_table: Table,
    pub cd: Cd,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateVersion {
    pub version_number: u64,
    pub target_id: Uuid,
    pub updater_id: Uuid,
    pub time_stamp: NaiveDateTime,
    pub json: Update,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    Cd(CdVersion),
    Update(UpdateVersion),
}
