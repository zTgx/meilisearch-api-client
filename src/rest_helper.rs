use serde_json::{Value};
use actix_web::{    
    http::{
        StatusCode,
    },
    client::Client
};
use crate::error::ServiceError;
use crate::constants;
use actix_web::body::Body;

// Get method
pub async fn get(url: String) -> Result<Value, ServiceError> {
    let client = Client::default();
    let request = client.get(url);

    // Set Headers
    let request = request.set_header("Content-Type", "application/json");
    
    let response = request.send().await;
    match response {
        Ok(mut res) => {
            match res.json::<Value>().await {
                Ok(value) => Ok(value),
                Err(err)  => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())) 
            }
        },
        Err(_err) => {
            Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
        }
    }
}

// Post method
pub async fn post(url: String, body: String) -> Result<Value, ServiceError> {
    let client = Client::default();
    let request = client.post(url);

    // Set Headers
    let request = request.set_header("Content-Type", "application/json");

    let response = request.send_body( Body::from_slice(body.as_bytes()) ).await;
    match response {
        Ok(mut res) => {
            match res.json::<Value>().await {
                Ok(value) => Ok(value),
                Err(err)  => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}

// Put method
pub async fn put(url: String, body: String) -> Result<Value, ServiceError> {

    let client = Client::default();
    let request = client.put(url);

    // Set Headers
    let request = request.set_header("Content-Type", "application/json");

    let response = request.send_body( Body::from_slice(body.as_bytes()) ).await;
    match response {
        Ok(mut res) => {
            match res.json::<Value>().await {
                Ok(value) => { println!("value: {:?}", value); Ok(value) },
                Err(err)  => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}
