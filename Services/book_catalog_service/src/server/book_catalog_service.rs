use std::sync::Arc;
use mediator::{AsyncMediator, DefaultAsyncMediator};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use crate::GetPagedBooksQuery;

use crate::grpc::{GetBooksRequest, GetBooksResponse};
use crate::grpc::catalog_service_server::CatalogService;

pub struct BookCatalogServiceImpl {
    mediator: Arc<Mutex<DefaultAsyncMediator>>,
}

impl BookCatalogServiceImpl {
    pub fn new(mediator: Arc<Mutex<DefaultAsyncMediator>>) -> Self {
        BookCatalogServiceImpl {
            mediator
        }
    }
}

#[tonic::async_trait]
impl CatalogService for BookCatalogServiceImpl {
    type GetBooksStream = ReceiverStream<Result<GetBooksResponse, Status>>;

    async fn get_books(&self, request: Request<GetBooksRequest>)
                       -> Result<Response<Self::GetBooksStream>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let mut mediator = self.mediator.lock().await;
        let books = mediator
            .send(GetPagedBooksQuery {
                page: request.get_ref().page,
                page_size: request.get_ref().page_size,
            })
            .await;

        match books {
            Ok(books) => {
                let (tx, rx) = mpsc::channel(100);
                tokio::spawn(async move {
                    for book in books.unwrap() {
                        tx.send(Ok(
                            GetBooksResponse {
                                book: Some(book),
                            }))
                            .await
                            .unwrap();
                    }
                });

                Ok(Response::new(ReceiverStream::new(rx)))
            }
            Err(_) => Err(Status::internal("Error")),
        }
    }
}