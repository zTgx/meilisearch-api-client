pub mod client;
mod indexes;
mod rest_helper;
mod error;
mod constants;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: usize,
}

impl Config {
    pub fn new(host: String, port: usize) -> Self {
        Config {
            host,
            port
        }
    }

    pub fn get_url(&self) -> String {
        self.host.as_str().to_owned() + ":" + self.port.to_string().as_str()
    }
}

// Index
#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub name: String,
    pub uid : String,

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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateIndexRequest {
    pub uid: String,
    
    #[serde(rename="primaryKey")]
    pub primary_key: Option<String>
}
impl CreateIndexRequest {
    pub fn new(uid: String, primary_key: Option<String>) -> Self {
        CreateIndexRequest {
            uid,
            primary_key
        }
    }
}
