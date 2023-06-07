use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Problem {
    pub id: Uuid,
    pub department_id: Uuid,
    pub title: String,
    pub description: String,
}
