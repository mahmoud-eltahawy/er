use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Employee {
    pub id: Uuid,
    pub department_id: Uuid,
    pub position: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub card_id: i64,
    pub password: String,
}
