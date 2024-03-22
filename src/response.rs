use crate::model::Todo;

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct TodoResponse {
    pub status: String,
    pub todo: Todo,
}

#[derive(Debug, serde::Serialize)]
pub struct TodoListResponse {
    pub status: String,
    pub results: Vec<Todo>,
    pub count: usize,
}
