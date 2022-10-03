use mongodb::Client;
use crate::domain::core::i_repository::IRepository;
use async_trait::async_trait;
use tokio_stream::StreamExt;
use crate::domain::book::book_entity::Book;
use crate::infrastructure::book::book_model::BookModel;
use crate::infrastructure::core::errors::InfrastructureError;

#[derive(Clone, Debug)]
pub struct BookRepository {
    client: Client,
    db_name: String,
    collection_name: String,
}

impl BookRepository {
    pub async fn new(mongodb_uri: String, db_name: String, collection_name: String) -> Self {
        let mongodb_client = Client::with_uri_str(&mongodb_uri)
            .await
            .expect("Failed to initialize MongoDB client.");

        BookRepository {
            client: mongodb_client,
            db_name,
            collection_name,
        }
    }

    fn get_books_collection(&self) -> mongodb::Collection<BookModel> {
        self.client
            .database(&self.db_name)
            .collection(&self.collection_name)
    }
}

#[async_trait]
impl IRepository<Book> for BookRepository {
    type Error = InfrastructureError;

    //TODO: do not fail the entire transaction if one book entity is invalid
    async fn get_paged(&self, page: u32, page_size: u32) -> Result<Option<Vec<Book>>, Self::Error> {
        let collection = self.get_books_collection();

        let skip = (page - 1) * page_size;
        let mut cursor = collection
            .find(None, None)
            .await
            .map_err(|e| InfrastructureError::MongoDb {
                message: e.to_string(),
            })?;

        let mut books = Vec::new();
        let mut count = 0;
        while let Some(result) = cursor.next().await {
            if count >= skip && count < skip + page_size {
                let book = result
                    .map_err(|e| InfrastructureError::MongoDb {
                        message: e.to_string(), 
                    })?;
                
                books.push(Book::try_from(book)
                    .map_err(|e| InfrastructureError::InvalidEntityFound {
                        message: e.to_string(), 
                    })?
                );
            }
            count += 1;
        }

        Ok(Some(books))
    }
}