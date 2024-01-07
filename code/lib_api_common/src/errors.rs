use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use sqlx::Error;

pub struct ApiError {
    pub code: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn bad_request(message: String) -> Self {
        ApiError { code: StatusCode::BAD_REQUEST, message }
    }
    pub fn not_found(message: String) -> Self {
        ApiError { code: StatusCode::NOT_FOUND, message }
    }
    pub fn new_internal(message: String) -> Self {
        ApiError { code: StatusCode::INTERNAL_SERVER_ERROR, message }
    }

    pub fn from_database_error(message: &str, sqlx_error: sqlx::Error) -> ApiError {
        tracing::error!("Failed to get project with id {:?} . " , sqlx_error);

        let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
        match sqlx_error {
            Error::RowNotFound => {
                status_code = StatusCode::NOT_FOUND
            }
            _ => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        return ApiError { code: status_code, message: message.to_string() };
    }
}


pub fn is_db_row_not_found(sqlx_error: sqlx::Error) -> bool {
    return match sqlx_error {
        Error::RowNotFound => {
            true
        }
        _ => {
            false
        }
    }
}

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("ApiError", 2)?;
        state.serialize_field("code", &self.code.as_u16())?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap(); // Convert your error to a JSON string
        (self.code, body).into_response()
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}