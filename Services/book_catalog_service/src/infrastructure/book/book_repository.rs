use mongodb::Client;
use crate::domain::core::i_repository::IRepository;
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::doc;
use crate::domain::book::book_entity::Book;
use crate::infrastructure::book::book_model::BookModel;
use crate::infrastructure::core::errors::ServerErrorType;

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
    type Error = ServerErrorType;

    async fn get_paged(&self, page: u32, page_size: u32) -> Result<Option<Vec<Result<Book, Self::Error>>>, Self::Error> {
        let collection = self.get_books_collection();

        let skip = (page - 1) * page_size;
        let sort_by_title = mongodb::options::FindOptions::builder()
            .sort(doc! { "title": 1 })
            .limit((page * page_size) as i64)
            .build();

        let mut cursor = collection
            .find(None, sort_by_title)
            .await
            .map_err(|e| ServerErrorType::MongoDb {
                message: e.to_string(),
            })?;

        let mut books = Vec::new();
        let mut count = 0;
        while let Ok(Some(raw_book)) = cursor.try_next().await {

            if count >= skip && count < skip + page_size {
                books.push(Book::try_from(raw_book)
                    .map_err(|e| ServerErrorType::InvalidEntityFound {
                        message: e.to_string(), 
                    })
                );
            }

            count += 1;
            if count >= skip + page_size {
                break;
            }
        }

        match books.is_empty() {
            true => Ok(None),
            false => Ok(Some(books)),
        }
    }
}