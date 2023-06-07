use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{TableResponse, Wrapable};
pub use models::spare_part::SparePart;

impl Wrapable for SparePart {
    fn wrap(self) -> TableResponse {
        TableResponse::SparePart(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateSparePart {
    UpdateName(Uuid, String),
}
