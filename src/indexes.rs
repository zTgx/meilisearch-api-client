use actix_web::client::Client;
use serde::{Deserialize, Serialize};
//use serde_json::{Value};
use std::str;
use crate::Config;

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

// Get information about an index.
pub async fn get_index(config: &Config, uid: String) -> Result<Index, &'static str> {
    let url = to_url(config, uid); 
    let client = Client::default();
    let response = client.get(url).header("Content-Type", "application/json").send().await;
    match response {
        Ok(mut res) => {
            match res.json::<Index>().await {
                Ok(index) => Ok(index),
                Err(_err)  => { Err("Data currupt") }
            }
        },
        Err(err) => {
            // let err_str = "API - get_index: ".to_string() + format!("{:?}", err).as_str();
            // Err(err_str.as_str())
            Err("tmp get_index")
        }
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
