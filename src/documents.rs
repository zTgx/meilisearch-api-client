use crate::constants;
use crate::Config;
use crate::rest_helper;
use crate::error::ServiceError;
use actix_web::http::StatusCode;
use crate::{
    DocumentRequest,
    DocumentState,
    Document,
};

// Get One Document
pub async fn get_document<T: Document>(config: &Config, uid: String, did: String) -> Result<T, ServiceError> {
    let host_and_port = config.get_url();
    let url = host_and_port + constants::INDEXES + "/" + uid.as_str() + constants::DOCUMENTS + "/" + did.as_str();

    let res = rest_helper::get(url).await;
    match res {
        Ok(value) => {
            let index: Result<T, serde_json::error::Error> = serde_json::from_value(value) as Result<T, serde_json::error::Error>;
            match index {
                Ok(data) => Ok(data),
                Err(err) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(err)
    }
}

// Get All Documents
pub async fn get_documents<T: Document>(config: &Config, uid: String, offset: Option<usize>, limit: Option<usize>) -> Result<Vec<T>, ServiceError> {
    let host_and_port = config.get_url();
    let mut query = "?".to_owned();
    if let Some(offset) = offset {
        query = query + "offset=" + offset.to_string().as_str();
    }
    if let Some(limit) = limit {
        query = query + "limit=" + limit.to_string().as_str();
    }
    let url = host_and_port + constants::INDEXES + "/" + uid.as_str() + constants::DOCUMENTS + query.as_str();
    println!("url: {:?}", url);

    let res = rest_helper::get(url).await;
    match res {
        Ok(value) => {
            println!("value; {:?}", value);
            let index: Result<Vec<T>, serde_json::error::Error> = serde_json::from_value(value) as Result<Vec<T>, serde_json::error::Error>;
            match index {
                Ok(data) => Ok(data),
                Err(err) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(err)
    }
}

// Add OR Replace documents
pub async fn add_or_replace<T: Document>(config: &Config, document_req: DocumentRequest<T>) -> Result<DocumentState, ServiceError> {
    let host_and_port = config.get_url();
    let uid = document_req.uid;
    let url = host_and_port + constants::INDEXES + "/" + uid.as_str() + constants::DOCUMENTS;

    let body = serde_json::to_string(&document_req.documents).unwrap();
    let res = rest_helper::post(url, body).await;
    match res {
        Ok(value) => {
            let index: Result<DocumentState, serde_json::error::Error> = serde_json::from_value(value) as Result<DocumentState, serde_json::error::Error>;
            match index {
                Ok(data) => Ok(data),
                Err(err) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(err)
    }
}

/*
// Delete Index
pub async fn delete_index(config: &Config, delete_index_req: DeleteIndexRequest) -> Result<String, ServiceError> {
    let host_and_port = config.get_url();
    let url = host_and_port + constants::INDEXES + "/" + delete_index_req.uid.as_str();

    let res = rest_helper::delete(url, None).await;
    println!("res: {:?}", res);
    match res {
        Ok(value) => {
            println!("value; {:?}", value);
            let index: Result<String, serde_json::error::Error> = serde_json::from_value(value) as Result<String, serde_json::error::Error>;
            match index {
                Ok(data) => Ok(data),
                Err(err) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        },
        Err(err) => Err(err)
    }
}
*/
