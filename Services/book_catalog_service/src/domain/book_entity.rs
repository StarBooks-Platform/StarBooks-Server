use std::fmt;
use std::fmt::Formatter;
use domain_patterns::models::Entity;
use isbnid::isbn::ISBN;
use mongodb::bson::oid::ObjectId;
use crate::BookModel;
use crate::domain::author::Author;
use crate::domain::core::errors::ValidationError;
use crate::domain::core::vvos::RblStringVVO;
use crate::domain::publisher::Publisher;
use crate::infrastructure::book_model::Asset;

#[derive(Debug)]
pub struct Book {
    pub id: ObjectId,
    pub isbn: ISBNWrapper,
    pub title: RblStringVVO<5, 50>,
    pub authors: Option<Vec<Author>>,
    pub publisher: Publisher,
    pub short_description: RblStringVVO<10, 500>,
    pub long_description: RblStringVVO<50, 2000>,
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

        let title = RblStringVVO::try_from(value.title)
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

        let short_description = RblStringVVO::try_from(value.short_description)
            .map_err(|e| ValidationError {
            message: format!("BookModel must have a valid short_description: {} Book ISBN: {}",
                             e.message.as_str(),
                             value.isbn.as_str())
        })?;
        let long_description = RblStringVVO::try_from(value.long_description)
            .map_err(|e| ValidationError {
            message: format!("BookModel must have a valid long_description: {} Book ISBN: {}",
                             e.message.as_str(),
                             value.isbn.as_str())
        })?;

        let year = value.year;
        let num_pages = value.num_pages;
        let cover_image = Asset::get(&value.cover_image.unwrap_or_else(|| "default.jpg".to_string()))
            .unwrap_or_else(|| panic!("Failed to load cover image for book with ISBN: {}", value.isbn.as_str()))
            .data.to_vec();

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
            year,
            num_pages,
            cover_image: cover_image.into(),
        })
    }
}