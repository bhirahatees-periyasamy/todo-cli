use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub task : String,
    pub create_at:  DateTime<Utc>,
    pub update_at: DateTime<Utc>,
    pub is_completed: bool,
    pub is_deleted: bool 
}


impl Todo {
    pub fn new(task: String) -> Self {
        Todo { task, create_at: Utc::now(), update_at: Utc::now(), is_completed: false, is_deleted: false }
    }
}