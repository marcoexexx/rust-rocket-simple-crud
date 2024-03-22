use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct AppState {
    pub todo_db: Arc<Mutex<Vec<Todo>>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            todo_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
