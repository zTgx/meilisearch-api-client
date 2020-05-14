use crate::constants;
use crate::Config;
use crate::rest_helper;
use crate::error::ServiceError;
use actix_web::http::StatusCode;
use crate::{
    Index,
    Indexes,
    
    CreateIndexRequest,
    UpdateIndexRequest,
};

// Get All Indexes
pub async fn get_indexes(config: &Config) -> Result<Indexes, ServiceError>{
    let host_and_port = config.get_url();
    let url = host_and_port + constants::INDEXES;
    let response = rest_helper::get(url).await;
    match response {
        Ok(value) => {
            let indexes: Result<Vec<Index>, serde_json::error::Error> = serde_json::from_value(value) as Result<Vec<Index>, serde_json::error::Error>;
            match indexes {
                Ok(data) => Ok( Indexes::new(data) ),
                Err(err) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(err)
    }
}

// Get Index
pub async fn get_index(config: &Config, uid: &'static str) -> Result<Index, ServiceError> {
    let host_and_port = config.get_url();
    let url = host_and_port + constants::INDEXES + "/" + uid;
    let res = rest_helper::get(url).await;
    match res {
        Ok(value) => {
            let index: Result<Index, serde_json::error::Error> = serde_json::from_value(value) as Result<Index, serde_json::error::Error>;
            match index {
                Ok(data) => Ok(data),
                Err(err) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(err)
    }
}

// Create Index
pub async fn create_index(config: &Config, create_index_req: CreateIndexRequest) -> Result<Index, ServiceError> {
    let host_and_port = config.get_url();
    let url = host_and_port + constants::INDEXES;

    let body = serde_json::to_string(&create_index_req).unwrap();
    let res = rest_helper::post(url, body).await;
    match res {
        Ok(value) => {
            let index: Result<Index, serde_json::error::Error> = serde_json::from_value(value) as Result<Index, serde_json::error::Error>;
            match index {
                Ok(data) => Ok(data),
                Err(err) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(err)
    }
}

// Update Index
pub async fn update_index(config: &Config, update_index_req: UpdateIndexRequest) -> Result<Index, ServiceError> {
    let host_and_port = config.get_url();
    let url = host_and_port + constants::INDEXES + "/" + update_index_req.uid.as_str();

    let body = serde_json::to_string(&update_index_req).unwrap();
    let res = rest_helper::put(url, body).await;
    println!("update : {:?}", res);
    match res {
        Ok(value) => {
            let index: Result<Index, serde_json::error::Error> = serde_json::from_value(value) as Result<Index, serde_json::error::Error>;
            match index {
                Ok(data) => Ok(data),
                Err(err) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(err)
    }
}
