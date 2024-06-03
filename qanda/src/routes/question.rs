use axum::extract::Json;
use axum::extract::Query;
use axum::http::response;
use axum::http::StatusCode;
use axum::Extension;
use axum_macros::debug_handler;
use handle_errors::Error;
use serde_json::json;
use std::collections::HashMap;
use std::result;
use tracing::instrument;

use crate::types::pagination::extract_pagination;
use crate::types::pagination::Pagination;

use axum::{
    body::Body,
    extract::{Path, State},
    response::{IntoResponse, Response},
};

use crate::store::Store;
use crate::types::question::NewQuestion;
use crate::types::question::{Question, QuestionId};

/*
pub async fn get_questions(
    Query(params): Query<HashMap<String,String>>,
    Extension(store): Extension<Store>,
)-> impl IntoResponse{

    let pagination = match extract_pagination(params) {
        Ok(p) => p,
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "Failed to extract pagination: {:?}", e);
            return (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid pagination parameters"}))).into_response();
        }
    };


    match store.get_questions(pagination.limit, pagination.offset).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "Failed to query questions: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to fetch questions"}))).into_response()
        }
    }
}

 */

#[debug_handler]
pub async fn get_questions(
    Query(params): Query<HashMap<String, String>>,
    State(store): State<Store>,
) -> Result<Response, Error> {
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        pagination = extract_pagination(params)?;
    }

    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(Json(res).into_response()),
        Err(e) => Err(Error::DatabaseQueryError),
    }
}

pub async fn add_question(
    State(store): State<Store>,
    Json(new_question): Json<NewQuestion>,
) -> Result<Response, Error> {
    match store.add_question(new_question).await {
        Ok(_) => Ok((StatusCode::OK).into_response()),
        Err(e) => Err(Error::DatabaseQueryError),
    }
}

pub async fn update_question(
    Path(id): Path<i32>,
    State(store): State<Store>,
    Json(question): Json<Question>,
) -> Result<Response, Error> {
    match store.update_question(question, id).await {
        Ok(res) => Ok(Json(res).into_response()),
        Err(e) => Err(Error::DatabaseQueryError),
    }
}

pub async fn delete_question(
    Path(id): Path<i32>,
    State(store): State<Store>,
) -> Result<Response, Error> {
    match store.delete_question(id).await {
        Ok(_) => Ok((StatusCode::OK).into_response()),
        Err(e) => Err(Error::DatabaseQueryError),
    }
}
