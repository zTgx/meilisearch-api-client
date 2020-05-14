use std::sync::Arc;
use serde_json::{Value};
use actix_web::{    
    http::{
        StatusCode,
    },
    client::Client
};
use crate::error::ServiceError;
use crate::constants;

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

#[derive(Debug)]
pub struct Data<T>(Arc<T>);
impl<T> Data<T> {
    /// Create new `Data` instance.
    ///
    /// Internally `Data` type uses `Arc`. if your data implements
    /// `Send` + `Sync` traits you can use `web::Data::new()` and
    /// avoid double `Arc`.
    pub fn new(state: T) -> Data<T> {
        Data(Arc::new(state))
    }

    /// Get reference to inner app data.
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    /// Convert to the internal Arc<T>
    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

#[derive(Debug)]
pub struct Web<T> {
    pub data: Box<Data<T>>
}
impl<T> Web<T> {
    pub fn new(data: Box<Data<T>>) -> Self {
        Web {
            data,
        }
    }
}

impl <T> Web<T> {
    pub async fn post(&self, url: String) -> Result<Value, ServiceError> {
        use crate::CreateIndexRequest;
        use actix_web::body::Body;
        use serde_json::json;

        let client = Client::default();
        let request = client.post(url);

        // Set Headers
        let request = request.set_header("Content-Type", "application/json");

        let body = CreateIndexRequest { uid: "test".to_string(), primary_key: None };
        let response = request.send_body( Body::from_slice( json!(body).to_string().as_bytes())).await;
        println!("response: {:?}", response);
        Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}
