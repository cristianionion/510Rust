use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::RwLock;
use tracing_subscriber::fmt::format::FmtSpan;
use tower_http::cors::{Any, CorsLayer};


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

use crate::routes::answer::add_answer;
use crate::routes::question::{add_question, delete_question, get_questions, update_question};

async fn handle_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

#[tokio::main]
async fn main() {
    //   log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    //https://carlosmv.hashnode.dev/adding-logging-and-tracing-to-an-axum-app-rust
    // RUST_LOG is not recognized as the name of ......
    //    log::error!("This is an error!");
    //    log::info!("This is a info!");
    //    log::warn!("This is a warning!");

    /*
        let log = warp::log::custom(|info|{
            eprintln!(
                "{}  {}  {}",
                info.method(),
                info.path(),
                info.status(),
            );
        });

    */

    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "practical_rust_book=info,warp=error".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // the url might be a lil off, page 312
    let store = store::Store::new("postgres://postgres:1234@localhost:5432/qanda").await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot migrate DB");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/questions", get(get_questions))
        //.route("/questions/:id", get(get_question))
        .route("/question/add", post(add_question))
        .route("/update/:id", put(update_question))
        .route("/delete/:id", delete(delete_question))
        .route("/answer/add", post(add_answer))
        .with_state(store)
        .layer(cors)
        .fallback(handle_404);

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 4000);
    eprintln!("qa: serving {}", ip);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
