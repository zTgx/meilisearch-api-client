// Copyright 2019-2020 zTgx <beautifularea@163.com>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A well designed api client for MeiliSearch.
//!
//! CAVEAT: WIP
//!
//!
//! # Quick Start
//!
//! To get you started quickly, the easiest and highest-level way to create
//! index is to use [`create_index`]; 
//!
//! ```
//! use meilib::{Config, client::Client, CreateIndexRequest};
//!
//! #[actix_rt::main]
//! async fn main() -> std::io::Result<()> {
//!    let uid  = "demo".to_string();
//!    let name = "demoname".to_string();
//!
//!    // construct a request param
//!    let req_data = CreateIndexRequest { uid, name, primary_key: None};
//!
//!    // config contains MeiliSearch server's host and port
//!    let config = Config::new("http://127.0.0.1".to_string(), 7700);
//!
//!    // Client is api interface, using async/await.
//!    let res = Client::new(config).create_index(req_data).await;
//!    match res {
//!        Ok(index) => {
//!            println!("ceate index: {:?}", index);
//!        },
//!        Err(err) => {
//!            println!("err: {:?}", err);
//!        }
//!     }
//!     
//!     Ok(())
//! }
//!
//! ```
//! Output:
//! ```
//! {"name":"demoname","uid":"demo","createdAt":"2020-05-14T08:56:24.483670375Z","updatedAt":"2020-05-14T08:56:24.484410846Z","primaryKey":null}
//! ```
//!
//! # Installation
//! 
//! This crate requires a MeiliSearch server to run. See [here](https://docs.meilisearch.com/guides/advanced_guides/installation.html#download-and-launch) to install and run MeiliSearch.  
//! For the user guide and further documentation, can be found
//! [here](https://docs.meilisearch.com/)
//! 

mod indexes;
mod documents;

mod rest_helper;
mod constants;

/// Module containing all the api interfaces
pub mod client;

/// Module containing ServiceError
pub mod error;

use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct};

/// Including sever's host and port
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

/// Index structure
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

/// Collection of Indexes
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

/// Including create index's params
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

/// Including update index's params
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

/// Including delete index's params
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
