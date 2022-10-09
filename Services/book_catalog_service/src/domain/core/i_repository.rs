use async_trait::async_trait;
use domain_patterns::models::Entity;

/// A generic async trait that defines the interface for a repository upon a specific ```TEntity``` type and an appropriate ```Error``` type.
#[async_trait]
pub trait IRepository<TEntity: Entity>: Send + Sync {
    type Error;

    /// This function returns a limited list of entities of type ```TEntity``` or ```Self::Error``` in case of infrastructure errors.
    ///
    /// The actual list of entities is wrapped in an ```Option``` to allow for the possibility of an empty list.
    ///
    /// Also, every item in the list is a ```Result``` to allow for the possibility of an invalid entity.
    async fn get_paged(&self, page: u32, page_size: u32) -> Result<Option<Vec<Result<TEntity, Self::Error>>>, Self::Error>;
}