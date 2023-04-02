use crate::error::ServiceError;
use crate::Config;
use crate::{documents, indexes};
use crate::{CreateIndexRequest, DeleteIndexRequest, Index, Indexes, UpdateIndexRequest};
use crate::{Document, DocumentRequest, DocumentState};

pub struct Client {
    pub config: Config,
}

// Client constructor
impl Client {
    pub fn new(config: Config) -> Self {
        Client { config }
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

    pub async fn create_index(
        &self,
        create_index_req: CreateIndexRequest,
    ) -> Result<Index, ServiceError> {
        indexes::create_index(&self.config, create_index_req).await
    }

    pub async fn update_index(
        &self,
        update_index_req: UpdateIndexRequest,
    ) -> Result<Index, ServiceError> {
        indexes::update_index(&self.config, update_index_req).await
    }

    pub async fn delete_index(
        &self,
        delete_index_req: DeleteIndexRequest,
    ) -> Result<String, ServiceError> {
        indexes::delete_index(&self.config, delete_index_req).await
    }
}

// impl [documents] APIs
impl Client {
    pub async fn get_document<T: Document>(
        &self,
        uid: String,
        did: String,
    ) -> Result<T, ServiceError> {
        documents::get_document(&self.config, uid, did).await
    }

    pub async fn get_documents<T: Document>(
        &self,
        uid: String,
        offset: Option<usize>,
        limit: Option<usize>,
        attributes: Option<&str>,
    ) -> Result<Vec<T>, ServiceError> {
        documents::get_documents::<T>(&self.config, uid, offset, limit, attributes).await
    }

    pub async fn add_or_replace<T: Document>(
        &self,
        document_req: DocumentRequest<T>,
    ) -> Result<DocumentState, ServiceError> {
        documents::add_or_replace(&self.config, document_req).await
    }
}
