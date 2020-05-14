mod indexes;
mod rest_helper;
mod error;
mod constants;
pub mod client;

use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct};

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
    pub uid : String,
    pub name: String,

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

#[derive(Deserialize, Debug)]
pub struct CreateIndexRequest {
    pub uid: String,
    pub name: String,

    #[serde(rename="primaryKey")]
    pub primary_key: Option<String>
}
impl CreateIndexRequest {
    pub fn new(uid: String, name: String, primary_key: Option<String>) -> Self {
        CreateIndexRequest {
            uid,
            name,
            primary_key
        }
    }
}
impl Serialize for CreateIndexRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CreateIndexRequest", 3)?;

        state.serialize_field("uid", &self.uid)?;
        state.serialize_field("name", &self.name)?;

        if self.primary_key.is_some () {
            state.serialize_field("primaryKey", &self.primary_key)?;
        } 

        state.end()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateIndexRequest {
    pub uid: String,
    pub name: String,

    #[serde(rename="primaryKey")]
    pub primary_key: String,
}

impl UpdateIndexRequest {
    pub fn new(uid: String, name: String, primary_key: String) -> Self {
        UpdateIndexRequest {
            uid,
            name,
            primary_key
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteIndexRequest {
    pub uid: String,
}

impl DeleteIndexRequest {
    pub fn new(uid: String) -> Self {
        DeleteIndexRequest {
            uid
        }
    }
}
