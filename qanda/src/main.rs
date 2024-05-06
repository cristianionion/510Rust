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

    //https://www.programiz.com/rust/hashmap#:~:text=Change%20Elements%20of%20a%20HashMap,(%22Apple%22))%3B%20fruits.

    pub async fn add_q(&self, question: Question) {
        let mut questions = self.questions.write().await;
        questions.insert(question.id.clone(), question);
    }

    pub async fn update_q(&self, question: Question) {
        let mut questions = self.questions.write().await;
        questions.insert(question.id.clone(), question);
    }

    pub async fn delete_q(&self, question: Question) {
        let mut questions = self.questions.write().await;
        questions.remove(&question.id.clone());
    }

    pub async fn add_a(&self, answer: Answer) {
        let mut answers = self.answers.write().await;
        answers.insert(answer.id.clone(), answer);
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

async fn insert_question(State(store): State<Store>, Json(question): Json<Question>) {
    store.add_q(question).await;
}

async fn insert_answer(State(store): State<Store>, Json(answer): Json<Answer>) {
    store.add_a(answer).await;
}
async fn update_question(State(store): State<Store>, Json(question): Json<Question>) {
    store.update_q(question).await;
}

async fn delete_question(State(store): State<Store>, Json(question): Json<Question>) {
    store.delete_q(question).await;
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
        .route("/question/add", post(insert_question))
        .route("/update/:id", put(update_question))
        .route("delete/:id", delete(delete_question))
        .route("answer/add", post(insert_answer))
        .with_state(store)
        .fallback(handle_404);

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    eprintln!("qa: serving {}", ip);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
