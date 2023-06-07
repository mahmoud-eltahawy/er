
pub use models::permissions::PermissionName;

use super::{FromExt, Stringify};

const WRITE_DEPARTMENT_PROBLEM: &str = "WRITE_DEPARTMENT_PROBLEM";
const READ_DEPARTMENT_PROBLEMS: &str = "READ_DEPARTMENT_PROBLEMS";
const MODIFY_DEPARTMENT_PROBLEMS: &str = "MODIFY_DEPARTMENT_PROBLEMS";
const DEFINE_PROBLEM: &str = "DEFINE_PROBLEM";
const ACCESS_HISTORY_DEPARTMENT_PROBLEMS: &str = "ACCESS_HISTORY_DEPARTMENT_PROBLEMS";
const ACCESS_HISTORY_ALL_DEPARTMENTS_PROBLEMS: &str = "ACCESS_HISTORY_ALL_DEPARTMENTS_PROBLEMS";
const ACCESS_HISTORY_DEPARTMENT_DEPARTMENT_PROBLEMS: &str =
    "ACCESS_HISTORY_DEPARTMENT_DEPARTMENT_PROBLEMS";
const ACCESS_HISTORY_ALL_DEPARTMENTS_DEPARTMENT_PROBLEMS: &str =
    "ACCESS_HISTORy_ALL_DEPARTMENTS_DEPARTMENT_PROBLEMS";
const ACCESS_HISTORY_MACHINES: &str = "ACCESS_HISTORY_MACHINES";
const ACCESS_HISTORY_SPARE_PARTS: &str = "ACCESS_HISTORY_SPARE_PARTS";
const ACCESS_HISTORY_EMPLOYEES: &str = "ACCESS_HISTORY_EMPLOYEES";

impl FromExt for PermissionName {
    fn ext_from(value: String) -> Result<Self, String> {
        match value.as_str() {
            DEFINE_PROBLEM => Ok(PermissionName::DefineProblem),
            ACCESS_HISTORY_MACHINES => Ok(PermissionName::AccessHistoryMachines),
            WRITE_DEPARTMENT_PROBLEM => Ok(PermissionName::WriteDepartmentProblem),
            READ_DEPARTMENT_PROBLEMS => Ok(PermissionName::ReadDepartmentProblems),
            ACCESS_HISTORY_EMPLOYEES => Ok(PermissionName::AccessHistoryEmployees),
            ACCESS_HISTORY_SPARE_PARTS => Ok(PermissionName::AccessHistorySpareParts),
            MODIFY_DEPARTMENT_PROBLEMS => Ok(PermissionName::ModifyDepartmentProblems),
            ACCESS_HISTORY_DEPARTMENT_PROBLEMS => {
                Ok(PermissionName::AccessHistoryDepartmentProblems)
            }
            ACCESS_HISTORY_ALL_DEPARTMENTS_PROBLEMS => {
                Ok(PermissionName::AccessHistoryAllDepartmentsProblems)
            }
            ACCESS_HISTORY_DEPARTMENT_DEPARTMENT_PROBLEMS => {
                Ok(PermissionName::AccessHistoryDepartmentDepartmentProblems)
            }
            ACCESS_HISTORY_ALL_DEPARTMENTS_DEPARTMENT_PROBLEMS => {
                Ok(PermissionName::AccessHistoryAllDepartmentsDepartmentProblems)
            }
            _ => Err("undefined permission".to_string()),
        }
    }
}

impl Stringify for PermissionName {
    fn stringify(&self) -> String {
        match self {
            PermissionName::DefineProblem => DEFINE_PROBLEM.to_string(),
            PermissionName::AccessHistoryMachines => ACCESS_HISTORY_MACHINES.to_string(),
            PermissionName::WriteDepartmentProblem => WRITE_DEPARTMENT_PROBLEM.to_string(),
            PermissionName::ReadDepartmentProblems => READ_DEPARTMENT_PROBLEMS.to_string(),
            PermissionName::AccessHistoryEmployees => ACCESS_HISTORY_EMPLOYEES.to_string(),
            PermissionName::AccessHistorySpareParts => ACCESS_HISTORY_SPARE_PARTS.to_string(),
            PermissionName::ModifyDepartmentProblems => MODIFY_DEPARTMENT_PROBLEMS.to_string(),
            PermissionName::AccessHistoryDepartmentProblems => {
                ACCESS_HISTORY_DEPARTMENT_PROBLEMS.to_string()
            }
            PermissionName::AccessHistoryAllDepartmentsProblems => {
                ACCESS_HISTORY_ALL_DEPARTMENTS_PROBLEMS.to_string()
            }
            PermissionName::AccessHistoryDepartmentDepartmentProblems => {
                ACCESS_HISTORY_DEPARTMENT_DEPARTMENT_PROBLEMS.to_string()
            }
            PermissionName::AccessHistoryAllDepartmentsDepartmentProblems => {
                ACCESS_HISTORY_ALL_DEPARTMENTS_DEPARTMENT_PROBLEMS.to_string()
            }
        }
    }
}
