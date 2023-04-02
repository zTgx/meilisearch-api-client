use crate::constants;
use crate::error::ServiceError;
use actix_web::body::Body;
use actix_web::{client::Client, http::StatusCode};
use serde_json::Value;

// Get method
pub async fn get(url: String) -> Result<Value, ServiceError> {
    let client = Client::default();
    let request = client.get(url);

    // Set Headers
    let request = request.set_header("Content-Type", "application/json");

    let response = request.send().await;
    match response {
        Ok(mut res) => match res.json::<Value>().await {
            Ok(value) => Ok(value),
            Err(err) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )),
        },
        Err(_err) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
        )),
    }
}

// Post method
pub async fn post(url: String, body: String) -> Result<Value, ServiceError> {
    let client = Client::default();
    let request = client.post(url);

    // Set Headers
    let request = request.set_header("Content-Type", "application/json");

    let response = request.send_body(Body::from_slice(body.as_bytes())).await;
    match response {
        Ok(mut res) => {
            let tp = res.body().await;
            match serde_json::from_slice(&tp.unwrap()) {
                Ok(value) => Ok(value),
                Err(err) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    err.to_string(),
                )),
            }
        }
        Err(err) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        )),
    }
}

// Put method
pub async fn put(url: String, body: String) -> Result<Value, ServiceError> {
    let client = Client::default();
    let request = client.put(url);

    // Set Headers
    let request = request.set_header("Content-Type", "application/json");

    let response = request.send_body(Body::from_slice(body.as_bytes())).await;
    match response {
        Ok(mut res) => match res.json::<Value>().await {
            Ok(value) => Ok(value),
            Err(err) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )),
        },
        Err(err) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        )),
    }
}

// Delete method
pub async fn delete(url: String, body: Option<String>) -> Result<Value, ServiceError> {
    let client = Client::default();
    let request = client.delete(url);

    // Set Headers
    let request = request.set_header("Content-Type", "application/json");

    // has body ?
    // Delete Index    => 204 No Content
    // Delete Document => 202 Accepted
    // ????
    match body {
        Some(data) => {
            let response = request.send_body(Body::from_slice(data.as_bytes())).await;
            match response {
                Ok(mut res) => match res.json::<Value>().await {
                    Ok(value) => Ok(value),
                    Err(err) => Err(ServiceError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        err.to_string(),
                    )),
                },
                Err(err) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    err.to_string(),
                )),
            }
        }
        None => {
            let response = request.send().await;
            match response {
                Ok(mut res) => {
                    // println!("mut resp: {:?}", res.body().await);
                    match res.json::<Value>().await {
                        Ok(value) => Ok(value),
                        Err(err) => Err(ServiceError::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            err.to_string(),
                        )),
                    }
                }
                Err(err) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    err.to_string(),
                )),
            }
        }
    }
}
