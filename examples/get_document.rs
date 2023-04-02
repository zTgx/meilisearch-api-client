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
    let did = "movie_id".to_string();

    // You Need To Tell Engine what's the type of your data.
    let res = Client::new(config).get_document::<Movie>(uid, did).await;
    match res {
        Ok(document) => {
            println!("document: {:?}", document);
        },
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
    Ok(())
}

