use actix_web::client::Client;
use serde_json::{Value};

pub async fn get(url: String) -> Result<Value, &'static str> {
    let client = Client::default();
    let request = client.get(url);

    // Set Headers
    let request = request.set_header("Content-Type", "application/json");
    
    let response = request.send().await;
    match response {
        Ok(mut res) => {
            match res.json::<Value>().await {
                Ok(value) => Ok(value),
                Err(_err)  => { Err("Data currupt") }
            }
        },
        Err(_err) => {
            // let err_str = "API - get_index: ".to_string() + format!("{:?}", err).as_str();
            // Err(err_str.as_str())
            Err("tmp get_index")
        }
    }
}
