use std::sync::Arc;
use mediator::{AsyncMediator, DefaultAsyncMediator};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use crate::{GetPagedBooksQuery, GrpcConfiguration};

use crate::grpc::{GetBooksRequest, GetBooksResponse};
use crate::grpc::catalog_service_server::CatalogService;

pub struct BookCatalogServiceImpl {
    mediator: Arc<Mutex<DefaultAsyncMediator>>,
    grpc_config: GrpcConfiguration,
}

impl BookCatalogServiceImpl {
    pub fn new(mediator: Arc<Mutex<DefaultAsyncMediator>>, grpc_config: GrpcConfiguration) -> Self {
        BookCatalogServiceImpl {
            mediator,
            grpc_config,
        }
    }
}

#[tonic::async_trait]
impl CatalogService for BookCatalogServiceImpl {
    type GetBooksStream = ReceiverStream<Result<GetBooksResponse, Status>>;

    async fn get_books(&self, request: Request<GetBooksRequest>) -> Result<Response<Self::GetBooksStream>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let mut mediator = self.mediator.lock().await;
        let books = mediator
            .send(GetPagedBooksQuery {
                page: request.get_ref().page,
                page_size: request.get_ref().page_size,
            })
            .await;

        match books {
            // no mediator related error occurred
            Ok(books) => {
                // init buffer streaming for books
                let (tx, rx) = mpsc::channel(self.grpc_config.buffer_size as usize);
                match books {
                    // no server related error occurred
                    Ok(books) => {
                        match books {
                            // books found
                            Some(books) => {
                                tokio::spawn(async move {
                                    for book in books {
                                        let send_book_response = tx.send(Ok(
                                            GetBooksResponse {
                                                book: Some(book),
                                            }))
                                            .await;

                                        if send_book_response.is_err() {
                                            continue;
                                        }
                                    }
                                });
                                Ok(Response::new(ReceiverStream::new(rx)))
                            }
                            // no books found
                            None => Err(Status::not_found("No books found")),
                        }
                    }
                    // server related error occurred
                    Err(e) => Err(Status::internal(e.name())),
                }
            }
            // mediator related error occurred
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }
}