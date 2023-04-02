extern crate actix_web;
extern crate meilisearch_api_client;

use meilisearch_api_client::{client::Client, Config};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let uid = "demo2";
    let config = Config::new("http://127.0.0.1".to_string(), 7700);
    let res = Client::new(config).get_index(uid).await;
    match res {
        Ok(index) => {
            println!("index: {:?}", index);
        }
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
    Ok(())
}
