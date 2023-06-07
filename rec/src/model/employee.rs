use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{permissions::PermissionName, TableResponse, Wrapable};

pub use models::employee::Employee;

impl Wrapable for Employee {
    fn wrap(self) -> TableResponse {
        TableResponse::Employee(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateEmployee {
    UpdatePassword(Uuid, String),
    UpdateDepartment(Uuid, Uuid),
    Up(Uuid),
    Down(Uuid),
    ForbidAllPermissions(Uuid),
    ForbidPermission(Uuid, PermissionName),
    AllowPermission(Uuid, PermissionName),
}
