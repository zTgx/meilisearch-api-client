use crate::indexes;
use crate::Config;
use crate::error::ServiceError;
use crate::{
    Index, 
    Indexes,

    CreateIndexRequest,
    UpdateIndexRequest,
};

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
    pub async fn get_indexes(&self) -> Result<Indexes, ServiceError> {
        indexes::get_indexes(&self.config).await
    }
    
    pub async fn get_index(&self, uid: &'static str) -> Result<Index, ServiceError> {
        indexes::get_index(&self.config, uid).await
    }
    
    pub async fn create_index(&self, create_index_req: CreateIndexRequest) -> Result<Index, ServiceError> {
        indexes::create_index(&self.config, create_index_req).await
    }

    pub async fn update_index(&self, update_index_req: UpdateIndexRequest) -> Result<Index, ServiceError> {
        indexes::update_index(&self.config, update_index_req).await
    }
}
