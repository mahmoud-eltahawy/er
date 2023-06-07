use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SparePart {
    pub id: Uuid,
    pub name: String,
}
