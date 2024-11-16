use core::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use log::error;
use oauth2::{basic::BasicErrorResponseType, RequestTokenError, StandardErrorResponse};
use serde::Deserialize;
use reqwest::Error as ReqwestError;
use serde_json::json;
use std::error::Error as StdError;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> ApiError {
        ApiError {
            status_code,
            message,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<ReqwestError> for ApiError {
    fn from(error: ReqwestError) -> Self {
      ApiError::new(error.status().unwrap_or_else(|| reqwest::StatusCode::INTERNAL_SERVER_ERROR).as_u16(), error.to_string()) 
    }
}


impl<E> From<RequestTokenError<E, StandardErrorResponse<BasicErrorResponseType>>> for ApiError
where
    E: StdError + std::fmt::Debug + std::fmt::Display, // Ensure E implements StdError
{
    fn from(error: RequestTokenError<E, StandardErrorResponse<BasicErrorResponseType>>) -> Self {
        match error {
            RequestTokenError::ServerResponse(err_response) => ApiError::new(
                400,
                format!(
                    "OAuth2 server response error: {} - {}",
                    err_response.error(),
                    err_response
                        .error_description()
                        .unwrap_or(&"No details provided".to_string())
                ),
            ),
            RequestTokenError::Request(err) => {
                ApiError::new(500, format!("OAuth2 HTTP request error: {}", err))
            }
            RequestTokenError::Parse(err, body) => ApiError::new(
                500,
                format!(
                    "OAuth2 response parse error: {}. Response body: {}",
                    err,
                    String::from_utf8_lossy(&body)
                ),
            ),
            RequestTokenError::Other(err) => {
                ApiError::new(500, format!("OAuth2 other error: {}", err))
            }
        }
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(_, err) => ApiError::new(409, err.message().to_string()),
            DieselError::NotFound => ApiError::new(404, String::from("Record not found")),
            err => ApiError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => {
                error!("{}", self.message);
                String::from("Internal server error")
            }
        };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}
