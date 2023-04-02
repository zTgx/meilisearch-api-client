extern crate actix_web;
extern crate meilisearch_api_client;

use meilisearch_api_client::{client::Client, Config, UpdateIndexRequest};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let uid = "demo3".to_string();
    let name = "demoname".to_string();
    let primary_key = "demo3_key".to_string();
    let req_data = UpdateIndexRequest {
        uid,
        name,
        primary_key,
    };
    let config = Config::new("http://127.0.0.1".to_string(), 7700);
    let res = Client::new(config).update_index(req_data).await;
    match res {
        Ok(index) => {
            println!("update index: {:?}", index);
        }
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
    Ok(())
}
