use std::error::Error;
use crate::domain::i_repository::IRepository;
use crate::infrastructure::book_model::BookModel;
use crate::infrastructure::book_repository::BookRepository;

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

    // let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    // let client = Client::with_options(client_options)?;
    //
    // let database = client.database("catalog_db");
    // let collection = database.collection::<BookModel>("book");
    //
    // let mut cursor = collection.find(None, None).await?;
    // while let Some(result) = cursor.next().await {
    //     match result {
    //         Ok(document) => println!("{:?}", document),
    //         Err(e) => println!("Error {:?}", e),
    //     }
    // }

    let book_repository = BookRepository::new(
        String::from("mongodb://localhost:27017"),
        String::from("catalog_db"),
        String::from("book")
    ).await;

    let books = book_repository.get_paged(1, 10).await?;

    for book in books.unwrap() {
        println!("{:?}", book);
    }

    Ok(())
}