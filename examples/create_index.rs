extern crate meilib;
extern crate actix_web;

use meilib::{Config, client::Client, CreateIndexRequest};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let uid = "demo2".to_string();
    let name = "demoname".to_string();
    let req_data = CreateIndexRequest { uid, name, primary_key: None};
    let config = Config::new("http://127.0.0.1".to_string(), 7700);
    let res = Client::new(config).create_index(req_data).await;
    match res {
        Ok(index) => {
            println!("ceate index: {:?}", index);
        },
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
    Ok(())
}

