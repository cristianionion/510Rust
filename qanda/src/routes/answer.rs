use std::collections::HashMap;
use axum::http::StatusCode;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::store::Store;
use crate::types::{
    answer::{Answer, AnswerId},
    question::QuestionId,
};

pub async fn insert_answer(State(store): State<Store>, Json(answer): Json<Answer>) {
    store.add_a(answer).await;
}