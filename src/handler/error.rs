use sea_orm::DbErr;
use serde_json::json;
use std::{backtrace::Backtrace, sync::Arc};
use trillium::{async_trait, Conn, Handler, Status};
use trillium_api::{ApiConnExt, Error as ApiError};
use validator::ValidationErrors;

pub struct ErrorHandler;
#[async_trait]
impl Handler for ErrorHandler {
    async fn run(&self, conn: Conn) -> Conn {
        conn
    }

    async fn before_send(&self, mut conn: Conn) -> Conn {
        let Some(error) = conn
            .take_state::<ApiError>()
            .map(Error::from)
            .or_else(|| conn.take_state())
        else { return conn };

        match error {
            Error::AccessDenied => conn.with_status(Status::Forbidden).with_body(""),

            Error::NotFound => conn.with_status(Status::NotFound).with_body(""),

            Error::JsonError(e @ ApiError::UnsupportedMimeType { .. }) => conn
                .with_status(Status::NotAcceptable)
                .with_body(e.to_string()),

            Error::JsonError(ApiError::ParseError { path, message }) => conn
                .with_status(Status::BadRequest)
                .with_json(&json!({"path": path, "message": message})),

            Error::ValidationErrors(e) => conn.with_status(Status::BadRequest).with_json(&e),

            e => {
                let mut conn = conn.with_status(Status::InternalServerError);
                log::error!("{e}");
                if cfg!(debug_assertions) {
                    conn.with_body(e.to_string())
                } else {
                    conn.inner_mut().take_response_body();
                    conn
                }
            }
        }
    }
}

#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    #[error("Access denied")]
    AccessDenied,
    #[error(transparent)]
    DatabaseError(#[from] Arc<DbErr>),
    #[error("Not found")]
    NotFound,
    #[error(transparent)]
    JsonError(#[from] ApiError),
    #[error(transparent)]
    ValidationErrors(#[from] ValidationErrors),
}

impl From<DbErr> for Error {
    fn from(value: DbErr) -> Self {
        Self::DatabaseError(Arc::new(value))
    }
}

#[async_trait]
impl Handler for Error {
    async fn run(&self, conn: Conn) -> Conn {
        conn.with_state(self.clone())
            .with_state(Backtrace::capture())
    }
}
