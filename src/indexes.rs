use serde::{Deserialize, Serialize};
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
impl Indexes {
    pub fn new(indexes: Vec<Index>) -> Self {
        Indexes {
            indexes
        }
    }
}

// Indexes route
static INDEXES: &'static str = "/indexes";

// Get Index
pub async fn get_index(config: &Config, uid: &'static str) -> Result<Index, &'static str> {
    let host_and_port = config.get_url();
    let url = host_and_port + INDEXES + "/" + uid;
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
pub async fn get_indexes(config: &Config) -> Result<Indexes, &'static str>{
    let host_and_port = config.get_url();
    let url = host_and_port + INDEXES;
    let response = rest_helper::get(url).await;
    match response {
        Ok(value) => {
            let indexes: Result<Vec<Index>, serde_json::error::Error> = serde_json::from_value(value) as Result<Vec<Index>, serde_json::error::Error>;
            match indexes {
                Ok(data) => Ok( Indexes::new(data) ),
                Err(_err) => Err("get_indexes Debug")
            }
        },
        Err(_err) => {
            Err("Get Response Error")
        }
    }
}
