use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, EnumIter)]
pub enum PermissionName {
    DefineProblem,
    AccessHistoryMachines,
    WriteDepartmentProblem,
    ReadDepartmentProblems,
    AccessHistoryEmployees,
    AccessHistorySpareParts,
    ModifyDepartmentProblems,
    AccessHistoryDepartmentProblems,
    AccessHistoryAllDepartmentsProblems,
    AccessHistoryDepartmentDepartmentProblems,
    AccessHistoryAllDepartmentsDepartmentProblems,
}
