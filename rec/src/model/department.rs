use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{TableResponse, Wrapable};

pub use models::department::Department;

impl Wrapable for Department {
    fn wrap(self) -> TableResponse {
        TableResponse::Department(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateDepartment {
    SetBoss(Uuid, Uuid),
    ChangeBoss(Uuid),
    UpdateName(Uuid, String),
}
