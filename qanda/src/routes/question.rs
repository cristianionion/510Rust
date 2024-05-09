use axum::http::StatusCode;
use std::collections::HashMap;

use axum::{
    body::Body,
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};

use crate::store::Store;
use crate::types::question::{Question, QuestionId};

pub async fn get_questions(State(store): State<Store>) -> Response {
    let questions = store.questions.read().await;
    // add the pagination

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&*questions).unwrap()))
        .unwrap();
    response.into_response()
}

pub async fn get_question(State(store): State<Store>, Path(params): Path<QuestionId>) -> Response {
    match store.get_q(&params).await {
        Some(question) => question.into_response(),
        None => (StatusCode::NOT_FOUND, "404 NOT FOUND").into_response(),
    }
}

pub async fn insert_question(State(store): State<Store>, Json(question): Json<Question>) {
    store.add_q(question).await;
}

pub async fn update_question(State(store): State<Store>, Json(question): Json<Question>) {
    store.update_q(question).await;
}

pub async fn delete_question(State(store): State<Store>, Json(question): Json<Question>) {
    store.delete_q(question).await;
}
