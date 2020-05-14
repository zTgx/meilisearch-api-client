extern crate meilib;
extern crate actix_web;

use meilib::{Config, client::Client, DeleteIndexRequest};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let uid = "demo1".to_string();
    let req_data = DeleteIndexRequest { uid };
    let config = Config::new("http://127.0.0.1".to_string(), 7700);
    let res = Client::new(config).delete_index(req_data).await;
    match res {
        Ok(result) => {
            println!("delete result: {:?}", result);
        },
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
    Ok(())
}

