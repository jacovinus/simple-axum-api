use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use uuid::Uuid;

use crate::{
    model::{
        // todos
        QueryOptions,
        Todo,
        UpdateTodoSchema,
        UpdateUserSchema,
        // user
        User,
        UserDb,
        DB,
    },
    response::{
        // todos
        SingleTodoResponse,
        // user
        SingleUserResponse,
        TodoData,
        TodoListResponse,
        UserData,
        UserListResponse,
    },
};

// health check
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build a simple CRUD API in rust using AXUM";

    let json_response = serde_json::json!({
        "status":"success",
        "message":MESSAGE
    });

    Json(json_response)
}

// User handlers

// Fetch All Users

pub async fn users_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(user_db): State<UserDb>,
) -> impl IntoResponse {
    let users = user_db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let users: Vec<User> = users.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = UserListResponse {
        status: "success".to_string(),
        results: users.len(),
        users,
    };

    Json(json_response)
}

pub async fn create_user_handler(
    State(user_db): State<UserDb>,
    Json(mut body): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = user_db.lock().await;

    if let Some(user) = vec.iter().find(|user| user.username == body.username) {
        let error_response = serde_json::json!({
            "status":"fail",
        "message": format!("User with username '{}' already exists", user.username)

        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let user = body.to_owned();

    vec.push(body);

    let json_response = SingleUserResponse {
        status: "success".to_string(),
        data: UserData { user },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_user_handler(
    Path(id): Path<Uuid>,
    State(user_db): State<UserDb>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = user_db.lock().await;

    if let Some(user) = vec.iter().find(|user| user.id == Some(id.to_owned())) {
        let json_response = SingleUserResponse {
            status: "success".to_string(),
            data: UserData { user: user.clone() },
        };

        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({

    "status":"fail".to_string(),
    "message": format!("User {} not found", id)

    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn edit_user_handler(
    Path(id): Path<Uuid>,
    State(user_db): State<UserDb>,
    Json(body): Json<UpdateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = user_db.lock().await;

    if let Some(user) = vec.iter_mut().find(|user| user.id == Some(id.clone())) {
        let datetime = chrono::Utc::now();

        let username = body
            .username
            .to_owned()
            .unwrap_or_else(|| user.username.to_owned());
        let name = body.name.to_owned().unwrap_or_else(|| user.name.to_owned());

        let payload = User {
            id: user.id.to_owned(),
            name: if !name.is_empty() {
                name
            } else {
                user.username.to_owned()
            },
            username: if !username.is_empty() {
                username
            } else {
                user.name.to_owned()
            },

            createdAt: user.createdAt,
            updatedAt: Some(datetime),
        };

        *user = payload;

        let json_response = SingleUserResponse {
            status: "success".to_string(),
            data: UserData { user: user.clone() },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
                "status":"fail",
                "message": format!("User with id {} could not be updated",id)

        });
        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}

pub async fn delete_user_handler(
    Path(id): Path<Uuid>,
    State(user_db): State<UserDb>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = user_db.lock().await;

    if let Some(po) = vec.iter().position(|user| user.id == Some(id.clone())) {
        vec.remove(po);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({

        "status" : "fail",
        "message" : format!("User with id {} not found",id)

    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

// Fetch All Records

pub async fn todos_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let todos = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();
    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };
    Json(json_response)
}

// add a record

pub async fn create_todo_handler(
    State(db): State<DB>,
    Json(mut body): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.title == body.title) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with title '{}' already exists", todo.title)
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();

    vec.push(body);

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

// retrieve a record

pub async fn get_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.id == Some(id.to_owned())) {
        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };

        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

// edit a record

pub async fn edit_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(body): Json<UpdateTodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();

    let mut vec = db.lock().await;

    if let Some(todo) = vec.iter_mut().find(|todo| todo.id == Some(id.clone())) {
        let datetime = chrono::Utc::now();
        let title = body
            .title
            .to_owned()
            .unwrap_or_else(|| todo.title.to_owned());

        let content = body
            .content
            .to_owned()
            .unwrap_or_else(|| todo.content.to_owned());

        let completed = body.completed.unwrap_or(todo.completed.unwrap());

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
            completed: Some(completed),
            createdAt: todo.createdAt,
            updatedAt: Some(datetime),
        };
        *todo = payload;

        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
            "status":"fail",
            "message":format!("Todo with ID: {} not found", id)
        });
        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}

pub async fn delete_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();

    let mut vec = db.lock().await;

    if let Some(pos) = vec.iter().position(|todo| todo.id == Some(id.clone())) {
        vec.remove(pos);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({
    "status": "fail",
    "message": format!("Todo with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
