use std::error::Error;
use crate::server::book_catalog_service::BookCatalogServiceImpl;
use crate::grpc::catalog_service_server::CatalogServiceServer;

mod grpc;
mod server;
mod infrastructure;
mod domain;
mod application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:5001".parse().unwrap();
    let book_catalog_service = BookCatalogServiceImpl::default();

    println!("Server listening on {}", addr);

    let svc = CatalogServiceServer::new(book_catalog_service);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}