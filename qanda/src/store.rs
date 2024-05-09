use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId},
};


#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
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
       
       
        // temp insert functionality
        
        let id = QuestionId("temp".to_string());

        let question = Question{
            id: id,
            title: "tempTitle".to_string(),
            content: "tempContent".to_string(),
            tags: Some(vec!["temp".to_string()]),
        };

        questions.insert(question.id.clone(), question);
    }

    pub async fn update_q(&self, question: Question) {
        let mut questions = self.questions.write().await;
        
        
        // temp update functionality
        
        let id = QuestionId("update".to_string());

        let question = Question{
            id: id,
            title: "updateTitle".to_string(),
            content: "updateContent".to_string(),
            tags: Some(vec!["update".to_string()]),
        };

        questions.insert(question.id.clone(), question);
    }

    pub async fn delete_q(&self, question: Question) {
        let mut questions = self.questions.write().await;
        
        // temp delete functionality
        
        let id = QuestionId("1".to_string());

        let question = Question{
            id: id,
            title: "How?".to_string(),
            content: "Please Help!".to_string(),
            tags: Some(vec!["general".to_string()]),
        };

        questions.remove(&question.id.clone());
    }

    pub async fn add_a(&self, answer: Answer) {
        let mut answers = self.answers.write().await;
        answers.insert(answer.id.clone(), answer);
    }
}