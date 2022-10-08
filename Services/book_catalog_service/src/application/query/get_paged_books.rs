use std::sync::Arc;
use async_trait::async_trait;
use mediator::{AsyncRequestHandler, Request};
use tokio::sync::Mutex;
use crate::domain::book::book_entity::Book;
use crate::grpc::BookDto;
use crate::infrastructure::core::errors::ServerErrorType;
use crate::IRepository;

pub struct GetPagedBooksQuery {
    pub page: u32,
    pub page_size: u32,
}

impl Request<Result<Option<Vec<BookDto>>, ServerErrorType>> for GetPagedBooksQuery {}

pub struct GetPagedBooksHandler {
    repository: Arc<Mutex<dyn IRepository<Book, Error=ServerErrorType>>>,
}

impl GetPagedBooksHandler {
    pub fn new(repository: Arc<Mutex<dyn IRepository<Book, Error=ServerErrorType>>>) -> Self {
        GetPagedBooksHandler { repository }
    }
}

#[async_trait]
impl AsyncRequestHandler<GetPagedBooksQuery, Result<Option<Vec<BookDto>>, ServerErrorType>> for GetPagedBooksHandler {
    async fn handle(&mut self, req: GetPagedBooksQuery) -> Result<Option<Vec<BookDto>>, ServerErrorType> {
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

#[cfg(test)]
mod get_paged_books_unit_tests {
    use super::*;
    use std::sync::Arc;
    use mockall::mock;
    use async_trait::async_trait;
    use isbnid::isbn::ISBN;
    use mediator::AsyncRequestHandler;
    use tokio::sync::Mutex;
    use crate::domain::book::book_entity::{Book, IsbnWrapper};
    use crate::domain::book::publisher::Publisher;
    use crate::domain::core::vvos::{PositiveFloatVvo, PositiveIntVvo, RblStringVvo, ReleaseYearVvo};
    use crate::grpc::Genre;
    use crate::infrastructure::core::errors::ServerErrorType;
    use crate::IRepository;

    mock! {
        pub BookRepository {
            fn get_paged(&self, page: u32, page_size: u32) -> Result<Option<Vec<Result<Book, ServerErrorType>>>, ServerErrorType>;
        }
    }

    #[async_trait]
    impl IRepository<Book> for MockBookRepository {
        type Error = ServerErrorType;

        async fn get_paged(&self, page: u32, page_size: u32) -> Result<Option<Vec<Result<Book, Self::Error>>>, Self::Error> {
            self.get_paged(page, page_size)
        }
    }

    #[tokio::test]
    async fn when_invoking_a_get_paged_books_request_upon_a_list_of_books_then_a_list_of_valid_books_only_is_returned() {
        let mut repository = MockBookRepository::new();
        repository
            .expect_get_paged()
            .times(1)
            .returning(|_, _| {
                Ok(Some(
                    vec![
                        Ok(Book {
                            id: Default::default(),
                            isbn: IsbnWrapper(ISBN::new("978-3-16-148410-0").unwrap()),
                            title: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy".to_string()).unwrap(),
                            authors: None,
                            publisher: Publisher {
                                name: RblStringVvo::try_from("Pan Books".to_string()).unwrap(),
                                address: RblStringVvo::try_from("London".to_string()).unwrap(),
                            },
                            short_description: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy is a comedy science fiction series created by Douglas Adams.".to_string()).unwrap(),
                            long_description: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy is a comedy science fiction series created by Douglas Adams.".to_string()).unwrap(),
                            price: PositiveFloatVvo::try_from(10.0).unwrap(),
                            genre: Genre::Unknown,
                            year: ReleaseYearVvo::try_from(1979).unwrap(),
                            num_pages: PositiveIntVvo::try_from(224).unwrap(),
                            cover_image: None,
                        }),
                        Err(ServerErrorType::InvalidEntityFound { message: "Invalid book found".to_string() }),
                        Ok(Book {
                            id: Default::default(),
                            isbn: IsbnWrapper(ISBN::new("978-3-16-148410-0").unwrap()),
                            title: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy".to_string()).unwrap(),
                            authors: None,
                            publisher: Publisher {
                                name: RblStringVvo::try_from("Pan Books".to_string()).unwrap(),
                                address: RblStringVvo::try_from("London".to_string()).unwrap(),
                            },
                            short_description: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy is a comedy science fiction series created by Douglas Adams.".to_string()).unwrap(),
                            long_description: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy is a comedy science fiction series created by Douglas Adams.".to_string()).unwrap(),
                            price: PositiveFloatVvo::try_from(10.0).unwrap(),
                            genre: Genre::Unknown,
                            year: ReleaseYearVvo::try_from(1979).unwrap(),
                            num_pages: PositiveIntVvo::try_from(224).unwrap(),
                            cover_image: None,
                        }),
                        Err(ServerErrorType::InvalidEntityFound { message: "Invalid book found".to_string() }),
                        Ok(Book {
                            id: Default::default(),
                            isbn: IsbnWrapper(ISBN::new("978-3-16-148410-0").unwrap()),
                            title: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy".to_string()).unwrap(),
                            authors: None,
                            publisher: Publisher {
                                name: RblStringVvo::try_from("Pan Books".to_string()).unwrap(),
                                address: RblStringVvo::try_from("London".to_string()).unwrap(),
                            },
                            short_description: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy is a comedy science fiction series created by Douglas Adams.".to_string()).unwrap(),
                            long_description: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy is a comedy science fiction series created by Douglas Adams.".to_string()).unwrap(),
                            price: PositiveFloatVvo::try_from(10.0).unwrap(),
                            genre: Genre::Unknown,
                            year: ReleaseYearVvo::try_from(1979).unwrap(),
                            num_pages: PositiveIntVvo::try_from(224).unwrap(),
                            cover_image: None,
                        }),
                    ]
                ))
            });

        let mut handler = GetPagedBooksHandler::new(Arc::new(Mutex::new(repository)));
        let result = handler.handle(GetPagedBooksQuery { page: 0, page_size: 3 }).await.unwrap();

        assert_eq!(result.unwrap().len(), 3);
    }
}