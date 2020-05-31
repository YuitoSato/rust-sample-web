use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize,Serialize};

#[derive(Deserialize, Serialize)]
pub struct Todo {
    id: u32,
    content: String,
}

pub async fn list() -> impl Responder {
    let todos = vec![
        Todo {
            id: 1,
            content: "1".to_string()
        }
    ];
    HttpResponse::Ok().json(todos)
}

pub async fn find(path: web::Path<(u32,)>) -> impl Responder {
    let todo = Todo {
        id: path.0,
        content: "hello".to_string()
    };
    HttpResponse::Ok().json(todo)
}

pub async fn create(todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().json(todo.0)
}
