use std::sync::Arc;
use async_trait::async_trait;
use mediator::{AsyncRequestHandler, Request};
use tokio::sync::Mutex;
use crate::domain::book::book_entity::Book;
use crate::grpc::BookDto;
use crate::infrastructure::core::errors::ServerError;
use crate::IRepository;

pub struct GetPagedBooksQuery {
    pub page: u32,
    pub page_size: u32,
}

impl Request<Result<Option<Vec<BookDto>>, ServerError>> for GetPagedBooksQuery {}

pub struct GetPagedBooksHandler {
    repository: Arc<Mutex<dyn IRepository<Book, Error = ServerError>>>,
}

impl GetPagedBooksHandler {
    pub fn new(repository: Arc<Mutex<dyn IRepository<Book, Error = ServerError>>>) -> Self {
        GetPagedBooksHandler { repository }
    }
}

#[async_trait]
impl AsyncRequestHandler<GetPagedBooksQuery, Result<Option<Vec<BookDto>>, ServerError>> for GetPagedBooksHandler {
    async fn handle(&mut self, req: GetPagedBooksQuery) -> Result<Option<Vec<BookDto>>, ServerError> {
        let repository = self.repository.lock().await;

        // if there is any major error, pass it to the grpc server
        let books = repository
            .get_paged(req.page, req.page_size)
            .await?;

        // if there is no error, but no books are found, return None
        if books.is_none() {
            return Ok(None);
        }

        // return only valid books
        Ok(Some(books
            .unwrap()
            .into_iter()
            .filter_map(Result::ok)
            .map(|book| book.into())
            .collect()
        ))
    }
}
