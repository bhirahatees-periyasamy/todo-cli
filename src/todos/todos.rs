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