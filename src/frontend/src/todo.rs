use seed::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
    message: String,
}

impl Todo {
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
    }

    pub fn completed_at(&self) -> &Option<chrono::DateTime<chrono::Utc>> {
        &self.completed_at
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub async fn get_all() -> Vec<Todo> {
    Request::new("http://localhost:3000")
        .method(Method::Get)
        .fetch()
        .await
        .expect("Could not GET")
        .json::<Vec<Todo>>()
        .await
        .expect("Could not parse JSON")
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoPayload {
    pub id: uuid::Uuid,
    pub message: String,
    pub done: bool,
}

pub async fn update(payload: &UpdateTodoPayload) {
    Request::new("http://localhost:3000")
        .method(Method::Put)
        .json(payload)
        .expect("Could not stringify JSON")
        .fetch()
        .await
        .expect("Could not PUT");
}

#[derive(Serialize, Deserialize)]
pub struct NewTodoPayload {
    pub message: String,
}

pub async fn new(payload: &NewTodoPayload) {
    Request::new("http://localhost:3000")
        .method(Method::Post)
        .json(payload)
        .expect("Could not stringify JSON")
        .fetch()
        .await
        .expect("Could not POST");
}

#[derive(Serialize, Deserialize)]
pub struct DeleteTodoPayload {
    pub id: uuid::Uuid,
}

pub async fn delete(payload: &DeleteTodoPayload) {
    Request::new("http://localhost:3000")
        .method(Method::Delete)
        .json(payload)
        .expect("Could not stringify JSON")
        .fetch()
        .await
        .expect("Could not POST");
}
