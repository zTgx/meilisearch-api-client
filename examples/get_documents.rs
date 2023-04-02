extern crate meilisearch_api_client;
extern crate actix_web;

use serde::{Deserialize, Serialize};
use meilisearch_api_client::{Config, client::Client, Document};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    #[derive(Deserialize, Serialize, Debug)]
    struct Movie {
        id: String,
        title: String,
    }
    impl Document for Movie {
        type IDType = String;
        fn get_id(&self) -> &String {
            &self.id
        }
    }
    
    let config = Config::new("http://127.0.0.1".to_string(), 7700);
    
    let uid = "demo3".to_string();
    
    // default value Some(0)
    let offset = None;

    // default value Some(20)
    let limit = Some(2);

    // attributesToRetrieve default value * show all fields
    let attributes = None;//Some("title");

    // You Need To Tell Engine what's the type of your data.
    let res = Client::new(config).get_documents::<Movie>(uid, offset, limit, attributes).await;
    match res {
        Ok(documents) => {
            println!("documents: {:?}", documents);
        },
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
    Ok(())
}

