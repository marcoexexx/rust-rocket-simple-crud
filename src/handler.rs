use chrono::Utc;
use rocket::{http, response::status, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    model::{AppState, Todo, UpdateTodoSchema},
    response::{GenericResponse, TodoListResponse, TodoResponse},
};

#[get("/healthchecker")]
pub async fn health_checker_handler(
) -> Result<Json<GenericResponse>, http::Status> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response = GenericResponse {
        message: String::from(MESSAGE),
        status: String::from("success"),
    };

    Ok(Json(response))
}

#[get("/todos?<page>&<limit>")]
pub async fn todos_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<TodoListResponse>, http::Status> {
    let vec = data.todo_db.lock().expect("Unable to get state");

    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> =
        vec.clone().into_iter().skip(offset).take(limit).collect();

    let res = TodoListResponse {
        status: String::from("success"),
        results: todos.clone(),
        count: todos.len(),
    };

    Ok(Json(res))
}

#[post("/todos", data = "<body>")]
pub async fn create_todo_handler(
    mut body: Json<Todo>,
    data: &State<AppState>,
) -> Result<Json<TodoResponse>, status::Custom<Json<GenericResponse>>> {
    let mut vec = data.todo_db.lock().expect("Unable to get state");

    for todo in vec.iter() {
        if todo.title == body.title {
            let err_res = GenericResponse {
                status: String::from("fail"),
                message: format!(
                    "Todo with title: `{}` already exists",
                    todo.title
                ),
            };
            return Err(status::Custom(
                http::Status::Conflict,
                Json(err_res),
            ));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = false;
    body.created_at = Some(datetime);
    body.updated_at = Some(datetime);

    let todo = body.to_owned().clone();

    vec.push(body.into_inner());

    let res = TodoResponse {
        status: String::from("success"),
        todo: todo.into_inner(),
    };

    Ok(Json(res))
}

#[get("/todos/<id>")]
pub async fn get_todo_handler(
    id: String,
    data: &State<AppState>,
) -> Result<Json<TodoResponse>, status::Custom<Json<GenericResponse>>> {
    let vec = data.todo_db.lock().expect("Unable to get state");

    for todo in vec.iter() {
        if todo.id == Some(id.to_owned()) {
            let res = TodoResponse {
                status: String::from("success"),
                todo: todo.clone(),
            };

            return Ok(Json(res));
        }
    }

    let err_res = GenericResponse {
        status: String::from("fail"),
        message: format!("Todo with ID: `{}` not found", id),
    };

    Err(status::Custom(http::Status::NotFound, Json(err_res)))
}

#[patch("/todos/<id>", data = "<body>")]
pub async fn edit_todo_handler(
    id: String,
    body: Json<UpdateTodoSchema>,
    data: &State<AppState>,
) -> Result<Json<TodoResponse>, status::Custom<Json<GenericResponse>>> {
    let mut vec = data.todo_db.lock().expect("Unable to get state");

    for todo in vec.iter_mut() {
        if todo.id == Some(id.clone()) {
            let datetime = Utc::now();
            let title =
                body.title.to_owned().unwrap_or(todo.title.to_owned());
            let content =
                body.content.to_owned().unwrap_or(todo.content.to_owned());

            let payload = Todo {
                id: todo.id.to_owned(),
                title: if !title.is_empty() {
                    title
                } else {
                    todo.title.to_owned()
                },
                content: if !content.is_empty() {
                    content
                } else {
                    todo.content.to_owned()
                },
                completed: body.completed.unwrap_or(false),
                updated_at: Some(datetime),
                created_at: todo.created_at,
            };

            *todo = payload;

            let res = TodoResponse {
                status: String::from("success"),
                todo: todo.clone(),
            };
            return Ok(Json(res));
        }
    }

    let err_res = GenericResponse {
        status: String::from("success"),
        message: format!("Todo with ID: `{}` not found", id),
    };

    Err(status::Custom(http::Status::NotFound, Json(err_res)))
}

#[delete("/todos/<id>")]
pub async fn delete_todo_handler(
    id: String,
    data: &State<AppState>,
) -> Result<http::Status, status::Custom<Json<GenericResponse>>> {
    let mut vec = data.todo_db.lock().expect("Unable to get state");

    for todo in vec.iter_mut() {
        if todo.id == Some(id.clone()) {
            vec.retain(|todo| todo.id != Some(id.to_owned()));
            return Ok(http::Status::NoContent);
        }
    }

    let res = GenericResponse {
        status: String::from("fail"),
        message: format!("Todo with ID: `{}` not found", id),
    };

    Err(status::Custom(http::Status::NotFound, Json(res)))
}
