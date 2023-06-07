use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Machine {
    pub id: Uuid,
    pub name: String,
}
