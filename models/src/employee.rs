use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use uuid::Uuid;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, EnumIter)]
pub enum Position {
    User,
    SuperUser,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Employee {
    pub id: Uuid,
    pub department_id: Uuid,
    pub position: Position,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub card_id: i64,
    pub password: String,
}
