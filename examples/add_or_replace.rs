extern crate meilisearch_api_client;
extern crate actix_web;

use serde::{Deserialize, Serialize};
use meilisearch_api_client::{Config, client::Client, Document, DocumentRequest};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let uid = "demo3".to_string();

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

    let req_data = DocumentRequest { 
                        uid, 
                        documents: Some( vec![ Movie { id: "movie_id".to_string(), title: "movie_title".to_string() } ] ) 
                    };

    let config = Config::new("http://127.0.0.1".to_string(), 7700);
    let res = Client::new(config).add_or_replace(req_data).await;
    match res {
        Ok(state) => {
            println!("documents state: {:?}", state);
        },
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
    Ok(())
}

