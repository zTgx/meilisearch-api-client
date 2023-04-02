extern crate actix_web;
extern crate meilisearch_api_client;

use meilisearch_api_client::{client::Client, Config};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new("http://127.0.0.1".to_string(), 7700);
    let res = Client::new(config).get_indexes().await;
    match res {
        Ok(indexes) => {
            println!("all indexes: {:?}", indexes);
        }
        Err(err) => {
            println!("get_indexes' error: {:?}", err);
        }
    }
    Ok(())
}
