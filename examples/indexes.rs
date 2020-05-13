extern crate meilib;
extern crate actix_web;

use meilib::indexes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let res = indexes::list_all_indexes().await;
    println!("all indexes: {:?}", res);

    Ok(())
}

