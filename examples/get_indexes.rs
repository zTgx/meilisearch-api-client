extern crate meilib;
extern crate actix_web;

use meilib::{Config, client::Client};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new("http://127.0.0.1".to_string(), 7700);
    let res = Client::new(config).get_indexes().await;
    println!("all indexes: {:?}", res);

    Ok(())
}

