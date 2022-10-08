use async_trait::async_trait;
use domain_patterns::models::Entity;

#[async_trait]
pub trait IRepository<TEntity: Entity>: Send + Sync {
    type Error;

    async fn get_paged(&self, page: u32, page_size: u32) -> Result<Option<Vec<Result<TEntity, Self::Error>>>, Self::Error>;
}