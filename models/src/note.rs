use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShiftNote {
    pub id: Uuid,
    pub shift_id: Uuid,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: Uuid,
    pub content: String,
}
