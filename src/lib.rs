pub mod client;
pub mod indexes;

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
}


