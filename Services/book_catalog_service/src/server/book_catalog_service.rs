use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::grpc::{Book, GetBooksRequest, GetBooksResponse, Author, Genre};
use crate::grpc::catalog_service_server::CatalogService;

#[derive(Default)]
pub struct BookCatalogServiceImpl {}

#[tonic::async_trait]
impl CatalogService for BookCatalogServiceImpl {
    type GetBooksStream = ReceiverStream<Result<GetBooksResponse, Status>>;

    async fn get_books(&self, request: Request<GetBooksRequest>)
                       -> Result<Response<Self::GetBooksStream>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            for i in 0..10 {
                tx.send(Ok(GetBooksResponse {
                    book: Some(Book {
                        isbn: i.to_string(),
                        title: format!("Book {}", i),
                        publisher_name: format!("Publisher {}", i),
                        authors: vec![
                            Author {
                                first_name: format!("Author {}", i),
                                last_name: format!("Last {}", i),
                            },
                            Author {
                                first_name: format!("Author {}", i),
                                last_name: format!("Last {}", i),
                            }
                        ],
                        genre: Genre::Fiction as i32,
                        short_description: format!("Short description {}", i),
                        price: i as f64,
                        cover_image: vec![],
                        release_year: i,
                    })
                })).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}