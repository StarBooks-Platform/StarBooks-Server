use std::fmt;
use std::fmt::Formatter;
use domain_patterns::models::{Entity, ValueObject};
use isbnid::isbn::ISBN;
use mongodb::bson::oid::ObjectId;
use crate::domain::book::author::Author;
use crate::domain::book::publisher::Publisher;
use crate::domain::core::errors::ValidationError;
use crate::domain::core::vvos::RblStringVvo;
use crate::grpc::{BookDto, Genre};
use crate::infrastructure::book::book_model::{Asset, BookModel};

#[derive(Debug)]
pub struct Book {
    pub id: ObjectId,
    pub isbn: ISBNWrapper,
    pub title: RblStringVvo<5, 50>,
    pub authors: Option<Vec<Author>>,
    pub publisher: Publisher,
    pub short_description: RblStringVvo<10, 500>,
    pub long_description: RblStringVvo<50, 2000>,
    pub price: f64,
    pub genre: Genre,
    pub year: u32,
    pub num_pages: u32,
    pub cover_image: Option<Vec<u8>>,
}

pub struct ISBNWrapper(ISBN);

impl fmt::Debug for ISBNWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.isbn13().as_str())
    }
}

impl Entity for Book {
    fn id(&self) -> String {
        self.id.to_string()
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl TryFrom<BookModel> for Book {
    type Error = ValidationError;

    fn try_from(value: BookModel) -> Result<Self, Self::Error> {
        let id = value.id.ok_or(ValidationError {
            message: "BookModel must have an id".to_string()
        })?;
        let isbn = ISBN::new(value.isbn.as_str())
            .map_err(|_| ValidationError {
                message: format!("BookModel must have a valid isbn: {}", value.isbn.as_str())
            })?;

        let title = RblStringVvo::try_from(value.title)
            .map_err(|e| ValidationError {
                message: format!("BookModel must have a valid title: {} Book ISBN: {}",
                                 e.message.as_str(),
                                 value.isbn.as_str())
            })?;

        let authors: Vec<Result<Author, ValidationError>> = value.authors
            .unwrap_or_default()
            .into_iter()
            .map(|a| {
                Author::try_from(a)
            })
            .collect();

        for author in authors.iter() {
            match author {
                Ok(_) => continue,
                Err(e) => Err(ValidationError {
                    message: format!("BookModel must have valid authors: {} Book ISBN: {}",
                                     e.message.as_str(),
                                     value.isbn.as_str())
                })?
            }
        }

        let publisher = Publisher::try_from(value.publisher)
            .map_err(|e| ValidationError {
                message: format!("BookModel must have a valid publisher: {}", e.message.as_str())
            })?;

        let short_description = RblStringVvo::try_from(value.short_description)
            .map_err(|e| ValidationError {
                message: format!("BookModel must have a valid short_description: {} Book ISBN: {}",
                                 e.message.as_str(),
                                 value.isbn.as_str())
            })?;
        let long_description = RblStringVvo::try_from(value.long_description)
            .map_err(|e| ValidationError {
                message: format!("BookModel must have a valid long_description: {} Book ISBN: {}",
                                 e.message.as_str(),
                                 value.isbn.as_str())
            })?;

        let price = value.price;
        let genre: Genre = value.genre.into();
        let year = value.year;
        let num_pages = value.num_pages;
        let cover_image = match Asset::get(
            &value.cover_image.unwrap_or_default()
        ) {
            Some(asset) => Some(asset.data.to_vec()),
            None => Some(Asset::get("default.jpg")
                .unwrap()
                .data.to_vec())
        };

        Ok(Book {
            id,
            isbn: ISBNWrapper(isbn),
            title,
            authors: authors
                .into_iter()
                .map(|a| a.unwrap())
                .collect::<Vec<Author>>()
                .into(),
            publisher,
            short_description,
            long_description,
            price,
            genre,
            year,
            num_pages,
            cover_image,
        })
    }
}

impl From<Book> for BookDto {
    fn from(value: Book) -> Self {
        BookDto {
            isbn: value.isbn.0.isbn13(),
            title: value.title.value(),
            publisher_name: value.publisher.name.value(),
            authors: value.authors
                .unwrap_or_default()
                .into_iter()
                .map(crate::grpc::Author::from)
                .collect(),
            genre: value.genre.into(),
            short_description: value.short_description.value(),
            price: value.price,
            cover_image: value.cover_image.unwrap_or_default(),
            release_year: value.year,
        }
    }
}