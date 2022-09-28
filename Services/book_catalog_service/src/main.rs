use std::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tokio_stream::StreamExt;
use crate::infrastructure::book_model::BookModel;

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

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let database = client.database("catalog_db");
    let collection = database.collection::<BookModel>("book");

    let mut cursor = collection.find(None, None).await?;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => println!("{:?}", document),
            Err(e) => println!("Error {:?}", e),
        }
    }

    Ok(())
}