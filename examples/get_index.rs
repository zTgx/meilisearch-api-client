extern crate meilib;
extern crate actix_web;

use meilib::indexes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let uid = "movies".to_string();
    let res = indexes::get_index(uid).await;
    println!("index: {:?}", res);

    Ok(())
}

