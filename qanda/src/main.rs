use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::RwLock;

use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};

mod routes;
mod store;
mod types;

use crate::routes::answer::insert_answer;
use crate::routes::question::{
    get_questions,get_question,insert_question,update_question,delete_question,
};





async fn handle_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

#[tokio::main]
async fn main() {
    let store = store::Store::new();

    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions/:id", get(get_question))
        .route("/question/add", post(insert_question))
        .route("/update/:id", put(update_question))
        .route("/delete/:id", delete(delete_question))
        //.route("/answer/add", post(insert_answer))
        .with_state(store)
        .fallback(handle_404);

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    eprintln!("qa: serving {}", ip);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
