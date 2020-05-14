use crate::indexes;
use crate::Config;
use crate::error;
use crate::{Index, Indexes};

pub struct Client {
    pub config: Config,
}

// Client constructor
impl Client {
    pub fn new(config: Config) -> Self {
        Client {
            config
        }
    }
}

// impl [indexes] APIs
impl Client {
    pub async fn get_index(&self, uid: &'static str) -> Result<Index, error::ServiceError> {
        indexes::get_index(&self.config, uid).await
    }

    pub async fn get_indexes(&self) -> Result<Indexes, &'static str> {
        indexes::get_indexes(&self.config).await
    }
}
