extern crate meilib;
extern crate actix_web;

use serde::{Deserialize, Serialize};
use meilib::{Config, client::Client, Document};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
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
    
    let config = Config::new("http://127.0.0.1".to_string(), 7700);
    
    let uid = "demo3".to_string();
    
    // default value Some(0)
    let offset = None;

    // default value Some(20)
    let limit = Some(2);

    // attributesToRetrieve default value * show all fields
    let attributes = "title";

    // You Need To Tell Engine what's the type of your data.
    let res = Client::new(config).get_documents::<Movie>(uid, offset, limit, None).await;
    match res {
        Ok(documents) => {
            println!("documents: {:?}", documents);
        },
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
    Ok(())
}

