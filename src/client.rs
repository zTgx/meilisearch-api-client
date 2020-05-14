use crate::indexes;
use crate::Config;
use crate::error::ServiceError;
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
    pub async fn get_index(&self, uid: &'static str) -> Result<Index, ServiceError> {
        indexes::get_index(&self.config, uid).await
    }

    pub async fn get_indexes(&self) -> Result<Indexes, ServiceError> {
        indexes::get_indexes(&self.config).await
    }
}
