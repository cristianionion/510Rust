use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::types::{
    answer::{Answer, AnswerId},
    question::QuestionId,
};
use crate::{store::Store, types::answer::NewAnswer};

use handle_errors::Error;

pub async fn add_answer(
    State(store): State<Store>,
    Json(new_answer): Json<NewAnswer>,
) -> Result<Response, Error> {
    match store.add_answer(new_answer).await {
        Ok(_) => Ok((StatusCode::OK).into_response()),
        Err(e) => Err(Error::DatabaseQueryError),
    }
}
