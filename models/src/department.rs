use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Department {
    pub id: Uuid,
    pub boss_id: Option<Uuid>,
    pub name: String,
}
