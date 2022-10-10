use std::sync::Arc;
use async_trait::async_trait;
use mediator::{AsyncRequestHandler, Request};
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
    repository: Arc<dyn IRepository<Book, Error=ServerErrorType>>,
}

impl GetPagedBooksHandler {
    pub fn new(repository: Arc<dyn IRepository<Book, Error=ServerErrorType>>) -> Self {
        GetPagedBooksHandler { repository }
    }
}

#[async_trait]
impl AsyncRequestHandler<GetPagedBooksQuery, Result<Option<Vec<BookDto>>, ServerErrorType>> for GetPagedBooksHandler {
    async fn handle(&mut self, req: GetPagedBooksQuery) -> Result<Option<Vec<BookDto>>, ServerErrorType> {
        // if there is any major error, pass it to the grpc server
        let books = self.repository
            .get_all()
            .await?;

        // if there is no error, but no books are found, return None
        if books.is_none() {
            return Ok(None);
        }

        // return only valid books with respect to the pagination parameters
        Ok(Some(books
            .unwrap()
            .into_iter()
            .filter_map(Result::ok)
            .skip((req.page * req.page_size) as usize)
            .take(req.page_size as usize)
            .map(|book| book.into())
            .collect()
        ))
    }
}

#[cfg(test)]
mod get_paged_books_unit_tests {
    use std::iter::zip;
    use std::str::FromStr;
    use super::*;
    use std::sync::Arc;
    use mockall::mock;
    use async_trait::async_trait;
    use isbn::Isbn13;
    use mediator::AsyncRequestHandler;
    use crate::domain::book::book_entity::{Book, IsbnWrapper};
    use crate::domain::book::publisher::Publisher;
    use crate::domain::core::vvos::{PositiveFloatVvo, PositiveIntVvo, RblStringVvo, ReleaseYearVvo};
    use crate::grpc::Genre;
    use crate::infrastructure::core::errors::ServerErrorType;
    use crate::IRepository;

    fn get_list_of_raw_isbns() -> Vec<String> {
        vec![
            "978-3-16-148410-0".to_string(),
            "978-3-16-148410-0".to_string(),
            "978-3-16-148410-0".to_string(),
        ]
    }

    fn get_list_of_raw_books() -> Result<Option<Vec<Result<Book, ServerErrorType>>>, ServerErrorType> {
        Ok(Some(
            vec![
                Ok(Book {
                    id: Default::default(),
                    isbn: IsbnWrapper(Isbn13::from_str("978-3-16-148410-0").unwrap()),
                    title: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy 1".to_string()).unwrap(),
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
                    isbn: IsbnWrapper(Isbn13::from_str("978-3-16-148410-0").unwrap()),
                    title: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy 2".to_string()).unwrap(),
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
                    isbn: IsbnWrapper(Isbn13::from_str("978-3-16-148410-0").unwrap()),
                    title: RblStringVvo::try_from("The Hitchhiker's Guide to the Galaxy 3".to_string()).unwrap(),
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
    }

    mock! {
        pub BookRepository {
            fn get_all(&self) -> Result<Option<Vec<Result<Book, ServerErrorType>>>, ServerErrorType>;
        }
    }

    #[async_trait]
    impl IRepository<Book> for MockBookRepository {
        type Error = ServerErrorType;

        async fn get_all(&self) -> Result<Option<Vec<Result<Book, Self::Error>>>, Self::Error> {
            self.get_all()
        }
    }

    #[tokio::test]
    async fn when_invoking_a_get_paged_books_request_upon_a_list_of_books_then_a_list_of_valid_books_only_is_returned() {
        let mut repository = MockBookRepository::new();
        repository
            .expect_get_all()
            .times(1)
            .returning(|| {
                get_list_of_raw_books()
            });

        let mut handler = GetPagedBooksHandler::new(Arc::new(repository));
        let result = handler.handle(GetPagedBooksQuery { page: 0, page_size: 3 }).await.unwrap();

        assert_eq!(result.clone().unwrap().len(), 3);
        zip(result.unwrap().iter(), get_list_of_raw_isbns().iter()).for_each(|(book, isbn)| {
            assert_eq!(book.isbn.to_string(), isbn.to_string());
        });
    }

    #[tokio::test]
    async fn when_invoking_a_get_paged_books_request_upon_a_list_of_enough_valid_books_to_complete_the_requested_batch_size_then_the_proper_slice_of_books_is_returned() {
        let mut repository = MockBookRepository::new();
        repository
            .expect_get_all()
            .times(1)
            .returning(|| {
                get_list_of_raw_books()
            });

        let mut handler = GetPagedBooksHandler::new(Arc::new(repository));
        let result = handler.handle(GetPagedBooksQuery { page: 1, page_size: 1 }).await.unwrap();

        assert_eq!(result.clone().unwrap().len(), 1);
        assert_eq!(
            result.unwrap().first().unwrap().isbn.to_string(),
            get_list_of_raw_isbns().get(1).unwrap().to_string()
        );
    }
}