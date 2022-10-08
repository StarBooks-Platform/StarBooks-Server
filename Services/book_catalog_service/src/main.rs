use std::error::Error;
use std::sync::Arc;
use mediator::DefaultAsyncMediator;
use tokio::sync::Mutex;
use domain::core::i_repository::IRepository;
use crate::application::query::get_paged_books::{GetPagedBooksHandler, GetPagedBooksQuery};
use crate::grpc::catalog_service_server::CatalogServiceServer;
use crate::infrastructure::book::book_repository::BookRepository;
use crate::server::book_catalog_service::BookCatalogServiceImpl;
use config::{GrpcConfiguration, MongoConfiguration};

mod grpc;
mod server;
mod infrastructure;
mod domain;
mod application;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let grpc_config = Arc::new(
        envy::prefixed("GRPC_")
            .from_env::<GrpcConfiguration>()
            .expect("Please provide GRPC_HOST and GRPC_PORT environment variables")
    );

    let addr = grpc_config.uri().parse()?;

    let mongo_config = Arc::new(
        envy::prefixed("MONGO_")
            .from_env::<MongoConfiguration>()
            .expect("Please provide MONGO_HOST, MONGO_PORT, MONGO_DB, \
        MONGO_COLLECTION, MONGO_USER and MONGO_PASS environment variables")
    );

    let mongo_client = Arc::new(
        mongodb::Client::with_uri_str(&mongo_config.uri())
            .await
            .expect("Failed to initialize MongoDB client.")
    );

    let book_repository = Arc::new(
        BookRepository::new(mongo_client, mongo_config)
    );

    let mediator = Arc::new(
        Mutex::new(
            DefaultAsyncMediator::builder()
                .add_handler(
                    GetPagedBooksHandler::new(
                        book_repository
                    ))
                .build()
        ));

    let book_catalog_service = BookCatalogServiceImpl::new(
        mediator,
        grpc_config,
    );

    println!("Server listening on {}", addr);

    let catalog_service_server = CatalogServiceServer::new(book_catalog_service);
    tonic::transport::Server::builder()
        .add_service(catalog_service_server)
        .serve(addr)
        .await?;

    Ok(())
}