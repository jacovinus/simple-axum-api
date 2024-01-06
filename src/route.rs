use axum::{
    routing::get,
    Router,
};

use crate::{
    handler::{
        create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
        health_checker_handler, todos_list_handler,
        create_user_handler, delete_user_handler, edit_user_handler, users_list_handler, get_user_handler
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::todo_db();
    let user_db = model::user_db();

    Router::new().route("/api/healthcheck", get(health_checker_handler)).route(
            "/api/todos",
            get(todos_list_handler).post(create_todo_handler),
        ).route(
            "/api/todo/:id",
            get(get_todo_handler).patch(edit_todo_handler).delete(delete_todo_handler)
        ).with_state(db)
        .route(
            "/api/users",
            get(users_list_handler).post(create_user_handler),
            
        ).route(
            "/api/users/:id",
        get(get_user_handler).patch(edit_user_handler).delete(delete_user_handler)
        )
        .with_state(user_db)


}
