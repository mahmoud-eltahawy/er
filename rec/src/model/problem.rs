use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{TableResponse, Wrapable};
pub use models::problem::Problem;

impl Wrapable for Problem {
    fn wrap(self) -> TableResponse {
        TableResponse::Problem(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateProblem {
    UpdateTitle(Uuid, String),
    UpdateDescription(Uuid, String),
}
