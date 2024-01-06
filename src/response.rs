// Serialize todo data
use crate::model::{Todo, User};

use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

// user responses
#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Debug)]
pub struct SingleUserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Debug)]
pub struct UserListResponse {
    pub status: String,
    pub results: usize,
    pub users: Vec<User>,
}

// todo responses
#[derive(Serialize, Debug)]
pub struct TodoData {
    pub todo: Todo,
}

#[derive(Serialize, Debug)]
pub struct SingleTodoResponse {
    pub status: String,
    pub data: TodoData,
}

#[derive(Serialize, Debug)]
pub struct TodoListResponse {
    pub status: String,
    pub results: usize,
    pub todos: Vec<Todo>,
}
