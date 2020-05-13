use actix_web::client::Client;
use serde::{Deserialize, Serialize};
//use serde_json::{Value};
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub name: String,
    pub uid	: String,

    #[serde(rename="createdAt")]
    pub created_at: String,
    
    #[serde(rename="updatedAt")]
    pub updated_at: String,
    
    #[serde(rename="primaryKey")]
    pub primary_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Indexes {
    pub indexes: Vec<Index>
}

pub async fn list_all_indexes() -> Result<Indexes, &'static str>{
    let client = Client::default();
    let response = client
        .get("http://127.0.0.1:7700/indexes")
        .header("Content-Type", "application/json")
        .send().await;       

    match response {
        Ok(mut res) => {
            let body = res.body().await;
            let v: Vec<Index> = serde_json::from_str(str::from_utf8(&body.unwrap()).unwrap()).unwrap();
            
            Ok( Indexes { indexes: v } )
        },
        Err(_err) => {
            Err("Get Response Error")
        }
    }
}
