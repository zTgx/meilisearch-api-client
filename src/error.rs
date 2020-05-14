use serde::{Deserialize, Serialize};
use actix_web::http::StatusCode;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub code: u32,
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(code: u32, message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            code: code,
            message: message.to_string(),
            data,
        }
    }
}

#[derive(Debug)]
pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                code: http_status.as_u16() as u32,
                message,
                data: String::new(),
            }
        }
    }
}

