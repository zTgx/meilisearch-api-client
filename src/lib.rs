pub mod client;
mod indexes;
mod rest_helper;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: usize,
}

impl Config {
    pub fn new(host: String, port: usize) -> Self {
        Config {
            host,
            port
        }
    }

    pub fn get_url(&self) -> String {
        self.host.as_str().to_owned() + ":" + self.port.to_string().as_str()
    }
}


