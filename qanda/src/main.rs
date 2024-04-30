use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::RwLock;

use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

//CORS Middleware - Introduction to Axum 0.5 by Brooks Builds on Youtube
//https://docs.rs/tower-http/0.5.2/tower_http/cors/index.html

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
    // 'made joke meaningful' commit
    pub async fn get_q(&self, index: &QuestionId) -> Option<Question> {
        // Rust Web Dev-Gruber, 4.2.1
        let questions = self.questions.read().await;
        let questions = &*questions;
        let question = questions.get(index)?;
        Some(question.to_owned())
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Deserialize, Clone, PartialEq, Eq, Hash, Debug, Serialize)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}
// 'made joke meaningful' commit

impl IntoResponse for &Question {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response()
    }
}

#[derive(Deserialize, Clone, PartialEq, Eq, Hash, Debug, Serialize)]
struct AnswerId(String);

#[derive(Deserialize, Debug, Clone, Serialize)]
struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Cannot Parse Parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing Parameter"),
            Error::QuestionNotFound => write!(f, "Question Not Found"),
        }
    }
}

async fn get_questions(State(store): State<Store>) -> Response {
    let questions = store.questions.read().await;
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&*questions).unwrap()))
        .unwrap();
    response.into_response()
}

async fn get_question(State(store): State<Store>, Path(params): Path<QuestionId>) -> Response {
    match store.get_q(&params).await {
        Some(question) => question.into_response(),
        None => (StatusCode::NOT_FOUND, "404 NOT FOUND").into_response(),
    }
}

async fn handle_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

#[tokio::main]
async fn main() {
    let store = Store::new();

    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions/:id", get(get_question))
        .with_state(store)
        .fallback(handle_404);

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    eprintln!("qa: serving {}", ip);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
