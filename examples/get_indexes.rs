extern crate meilib;
extern crate actix_web;

use meilib::indexes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let res = indexes::get_indexes().await;
    println!("all indexes: {:?}", res);

    Ok(())
}

