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

// Get All Indexes
pub async fn get_indexes() -> Result<Indexes, &'static str>{
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

// Get information about an index.
pub async fn get_index(uid: String) -> Result<Index, &'static str> {
    let client = Client::default();
    let url = "http://127.0.0.1:7700/indexes".to_string() + "/" + uid.as_str();
    let response = client.get(url).header("Content-Type", "application/json").send().await;
    match response {
        Ok(mut res) => {
            match res.json::<Index>().await {
                Ok(index) => Ok(index),
                Err(err)  => { Err("Data currupt") }
            }
        },
        Err(_err) => {
            Err("Bad response")
        }
    }
}
