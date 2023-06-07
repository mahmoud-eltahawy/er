pub mod department;
pub mod employee;
pub mod machine;
pub mod name;
pub mod note;
pub mod permissions;
pub mod problem;
pub mod shift;
pub mod shift_problem;
pub mod spare_part;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait FromExt {
    fn ext_from(value: String) -> Result<Self, String> where Self : Sized;
}

pub trait Stringify {
    fn stringify(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub enum TableRequest {
    DepartmentShift(TableCrud<shift::DepartmentShift, shift::UpdateDepartmentShift>),
    Machine(TableCrud<machine::Machine, machine::UpdateMachine>),
    Problem(TableCrud<problem::Problem, problem::UpdateProblem>),
    Employee(TableCrud<employee::Employee, employee::UpdateEmployee>),
    SparePart(TableCrud<spare_part::SparePart, spare_part::UpdateSparePart>),
    Department(TableCrud<department::Department, department::UpdateDepartment>),
    ShiftProblem(TableCrud<shift_problem::ShiftProblem, shift_problem::UpdateShiftProblem>),
}

#[derive(Serialize, Deserialize)]
pub enum Update {
    DepartmentShift(shift::UpdateDepartmentShift),
    Machine(machine::UpdateMachine),
    Problem(problem::UpdateProblem),
    Employee(employee::UpdateEmployee),
    SparePart(spare_part::UpdateSparePart),
    Department(department::UpdateDepartment),
    ShiftProblem(shift_problem::UpdateShiftProblem),
}

#[derive(Serialize, Deserialize)]
pub enum TableResponse {
    DepartmentShift(shift::DepartmentShift),
    Machine(machine::Machine),
    Problem(problem::Problem),
    Employee(employee::Employee),
    SparePart(spare_part::SparePart),
    Department(department::Department),
    ShiftProblem(shift_problem::ShiftProblem),
    Done,
    Err(String),
}

pub trait Wrapable {
    fn wrap(self) -> TableResponse;
}

#[derive(Serialize, Deserialize)]
pub enum TableCrud<TableStruct: Wrapable, UpdateTableEnum> {
    Create(Environment<TableStruct>),
    Read(Uuid),
    Update(Environment<UpdateTableEnum>),
    Delete(Environment<Uuid>, Option<Uuid>),
}

#[derive(Serialize, Deserialize)]
pub struct Environment<T> {
    pub updater_id: Uuid,
    pub time_stamp: NaiveDateTime,
    pub target: T,
}
