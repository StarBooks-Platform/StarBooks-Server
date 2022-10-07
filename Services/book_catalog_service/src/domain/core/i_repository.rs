use async_trait::async_trait;
use domain_patterns::models::Entity;
use mockall::automock;

// TODO: mock this trait
#[async_trait]
// #[automock]
pub trait IRepository<TEntity: Entity>: Send + Sync {
    type Error;

    async fn get_paged(&self, page: u32, page_size: u32) -> Result<Option<Vec<Result<TEntity, Self::Error>>>, Self::Error>;
}