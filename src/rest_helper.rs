use serde_json::{Value};
use actix_web::{    
    http::{
        StatusCode,
    },
    client::Client
};
use crate::error;
use crate::constants;

pub async fn get(url: String) -> Result<Value, error::ServiceError> {
    let client = Client::default();
    let request = client.get(url);

    // Set Headers
    let request = request.set_header("Content-Type", "application/json");
    
    let response = request.send().await;
    match response {
        Ok(mut res) => {
            match res.json::<Value>().await {
                Ok(value) => Ok(value),
                Err(err)  => Err(error::ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())) 
            }
        },
        Err(_err) => {
            Err(error::ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
        }
    }
}
