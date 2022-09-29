use std::sync::Arc;
use async_trait::async_trait;
use mediator::{AsyncRequestHandler, Request};
use tokio::sync::Mutex;
use crate::domain::book::book_entity::Book;
use crate::grpc::BookDto;
use crate::infrastructure::core::errors::InfrastructureError;
use crate::IRepository;

pub struct GetPagedBooksQuery {
    pub page: u32,
    pub page_size: u32,
}

impl Request<Option<Vec<BookDto>>> for GetPagedBooksQuery {}

pub struct GetPagedBooksHandler {
    repository: Arc<Mutex<dyn IRepository<Book, Error = InfrastructureError>>>,
}

impl GetPagedBooksHandler {
    pub fn new(repository: Arc<Mutex<dyn IRepository<Book, Error = InfrastructureError>>>) -> Self {
        GetPagedBooksHandler { repository }
    }
}

#[async_trait]
impl AsyncRequestHandler<GetPagedBooksQuery, Option<Vec<BookDto>>> for GetPagedBooksHandler {
    async fn handle(&mut self, req: GetPagedBooksQuery) -> Option<Vec<BookDto>> {
        let repository = self.repository.lock().await;
        let books = repository.get_paged(req.page, req.page_size).await.unwrap();
        let books = books.unwrap();

        let books = books.into_iter().map(|book| book.into()).collect();
        Some(books)
    }
}
