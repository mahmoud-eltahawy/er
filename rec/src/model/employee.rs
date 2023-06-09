use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{FromExt, Stringify};
use super::{permissions::PermissionName, TableResponse, Wrapable};

pub use models::employee::Employee;
pub use models::employee::Position;

const USER: &str = "USER";
const SUPER_USER: &str = "SUPER_USER";

impl FromExt for Position {
    fn ext_from(value: String) -> Result<Self, String> {
        match value.as_str() {
            USER => Ok(Position::User),
            SUPER_USER => Ok(Position::SuperUser),
            _ => Err("undefined permission".to_string()),
        }
    }
}

impl Stringify for Position {
    fn stringify(&self) -> String {
        match self {
            Position::User => USER.to_string(),
            Position::SuperUser => SUPER_USER.to_string()
        }
    }
}

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
