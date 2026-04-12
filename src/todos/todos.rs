use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Todo {
    pub task : String,
    pub create_at:  DateTime<Utc>,
    pub update_at: DateTime<Utc>,
    pub is_completed: bool,
    pub is_deleted: bool 
}

#[derive(Debug)]
pub struct TodoList {
    pub tasks: Vec<Todo>
}

impl Todo {
    pub fn new(task: String) -> Self {
        Todo { task, create_at: Utc::now(), update_at: Utc::now(), is_completed: false, is_deleted: false }
    }
}