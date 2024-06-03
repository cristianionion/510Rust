use axum::body::Body;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    DatabaseQueryError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::ParseError(_) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Failed to parse integer"))
                .unwrap(),
            Error::MissingParameters => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Missing parameters"))
                .unwrap(),

            Error::DatabaseQueryError => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Database Query Error"))
                .unwrap(),
        }
    }
}
