use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn with_id(id: i32, name: String, email: String) -> Self {
        Self {
            id: Some(id),
            name,
            email,
        }
    }
}
