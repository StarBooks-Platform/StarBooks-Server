use std::error::Error;
use std::sync::Arc;
use mediator::{AsyncMediator, DefaultAsyncMediator};
use tokio::sync::Mutex;
use domain::core::i_repository::IRepository;
use crate::application::query::get_paged_books::{GetPagedBooksHandler, GetPagedBooksQuery};
use crate::infrastructure::book::book_repository::BookRepository;

mod grpc;
mod server;
mod infrastructure;
mod domain;
mod application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let addr = "0.0.0.0:5001".parse().unwrap();
    // let book_catalog_service = BookCatalogServiceImpl::default();
    //
    // println!("Server listening on {}", addr);
    //
    // let svc = CatalogServiceServer::new(book_catalog_service);
    // tonic::transport::Server::builder()
    //     .add_service(svc)
    //     .serve(addr)
    //     .await?;

    let book_repository = BookRepository::new(
        String::from("mongodb://localhost:27017"),
        String::from("catalog_db"),
        String::from("book")
    ).await;

    let repository = Arc::new(Mutex::new(book_repository));
    let mut mediator = DefaultAsyncMediator::builder()
        .add_handler(GetPagedBooksHandler::new(repository))
        .build();

    let books = mediator
        .send(GetPagedBooksQuery {
            page: 1,
            page_size: 10,
        })
        .await
        .unwrap();

    for book in books.unwrap() {
        println!("{:?}", book.title);
    }

    Ok(())
}