use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{TableResponse, Wrapable};

pub use models::machine::Machine;

impl Wrapable for Machine {
    fn wrap(self) -> TableResponse {
        TableResponse::Machine(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateMachine {
    UpdateName(Uuid, String),
}
