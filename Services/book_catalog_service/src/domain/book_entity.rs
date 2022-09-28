use domain_patterns::models::Entity;
use isbnid::isbn::ISBN;
use mongodb::bson::Uuid;
use crate::BookModel;
use crate::domain::rblan_string_vvo::{RblanStringVVO, ValidationError};
use crate::infrastructure::book_model::{AuthorModel, PublisherModel};

pub struct Book {
    pub id: Uuid,
    pub isbn: ISBN,
    pub title: RblanStringVVO<5, 50>,
    pub authors: Option<Vec<Author>>,
    pub publisher: Publisher,
    pub short_description: RblanStringVVO<10, 100>,
    pub long_description: RblanStringVVO<50, 1000>,
    pub year: u32,
    pub num_pages: u32,
    pub cover_image: Option<Vec<u8>>,
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

        let title = RblanStringVVO::try_from(value.title)
            .map_err(|e| ValidationError {
            message: format!("BookModel must have a valid title: {}", e.message.as_str())
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
                Err(e) => Err(e.clone())?
            }
        }

        let publisher = Publisher::try_from(value.publisher)
            .map_err(|e| ValidationError {
            message: format!("BookModel must have a valid publisher: {}", e.message.as_str())
        })?;

        let short_description = RblanStringVVO::try_from(value.short_description)
            .map_err(|e| ValidationError {
            message: format!("BookModel must have a valid short_description: {}", e.message.as_str())
        })?;
        let long_description = RblanStringVVO::try_from(value.long_description)
            .map_err(|e| ValidationError {
            message: format!("BookModel must have a valid long_description: {}", e.message.as_str())
        })?;

        let year = value.year;
        let num_pages = value.num_pages;
        let cover_image = base64::decode(value.cover_image.unwrap_or_default().as_str())
            .map_err(|e| ValidationError {
            message: format!("BookModel must have a valid cover_image: {}", e)
        })?;

        Ok(Book {
            id,
            isbn,
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

pub struct Author {
    pub first_name: RblanStringVVO<5, 50>,
    pub last_name: RblanStringVVO<5, 50>,
}

impl TryFrom<AuthorModel> for Author {
    type Error = ValidationError;

    fn try_from(value: AuthorModel) -> Result<Self, Self::Error> {
        let first_name = RblanStringVVO::try_from(value.first_name)
            .map_err(|e| ValidationError {
            message: format!("AuthorModel.first_name is invalid: {}", e.message.as_str())
        })?;
        let last_name = RblanStringVVO::try_from(value.last_name)
            .map_err(|e| ValidationError {
            message: format!("AuthorModel.last_name is invalid: {}", e.message.as_str())
        })?;

        Ok(Author {
            first_name,
            last_name,
        })
    }
}

pub struct Publisher {
    pub name: RblanStringVVO<5, 100>,
    pub address: RblanStringVVO<5, 100>,
}

impl TryFrom<PublisherModel> for Publisher {
    type Error = ValidationError;

    fn try_from(value: PublisherModel) -> Result<Self, Self::Error> {
        let name = RblanStringVVO::try_from(value.name)
            .map_err(|e| ValidationError {
            message: format!("PublisherModel.name is invalid: {}", e.message.as_str())
        })?;
        let address = RblanStringVVO::try_from(value.address)
            .map_err(|e| ValidationError {
            message: format!("PublisherModel.address is invalid: {}", e.message.as_str())
        })?;

        Ok(Publisher {
            name,
            address,
        })
    }
}