use actix_web::client::Client;
use serde::{Deserialize, Serialize};
//use serde_json::{Value};
use std::str;
use crate::Config;
use crate::rest_helper;

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

// Indexes route
static INDEXES: &'static str = "/indexes";

// Util concat URL
fn to_url(config: &Config, uid: String) -> String {
    let host = config.host.to_owned();
    let port = config.port;

    host + ":" + port.to_string().as_str() + INDEXES + "/" + uid.as_str()
}

// Get Index
pub async fn get_index(config: &Config, uid: String) -> Result<Index, &'static str> {
    let url = to_url(config, uid); 
    let res = rest_helper::get(url).await;
    match res {
        Ok(value) => {
            let index: Result<Index, serde_json::error::Error> = serde_json::from_value(value) as Result<Index, serde_json::error::Error>;
            match index {
                Ok(data) => Ok(data),
                Err(_err) => {
                    Err("Debug")
                }
            }
        },
        Err(_err) => Err("value erro")
    }
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
