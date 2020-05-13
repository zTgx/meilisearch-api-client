use crate::indexes;
use crate::Config;

/* // trait not support asybc / await now.
pub trait IndexesAPI {
    type Index;
    async fn get_index(&self, uid: String) -> Result<Self::Index, &'static str>;
}
*/

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
    pub async fn get_index(&self, uid: String) -> Result<indexes::Index, &'static str> {
        indexes::get_index(&self.config, uid).await
    }  
}
