use std::fmt;
use std::fmt::Formatter;
use domain_patterns::models::{Entity, ValueObject};
use isbnid::isbn::ISBN;
use mongodb::bson::oid::ObjectId;
use crate::domain::book::author::Author;
use crate::domain::book::publisher::Publisher;
use crate::domain::core::errors::{ValidationError, ValidationErrorType};
use crate::domain::core::vvos::{PositiveFloatVvo, PositiveIntVvo, RblStringVvo, ReleaseYearVvo};
use crate::grpc::{AuthorDto, BookDto, Genre};
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
    pub price: PositiveFloatVvo,
    pub genre: Genre,
    pub year: ReleaseYearVvo<1900>,
    pub num_pages: PositiveIntVvo,
    pub cover_image: Option<Vec<u8>>,
}

pub struct ISBNWrapper(ISBN);

impl fmt::Debug for ISBNWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.isbn13())
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
    type Error = ValidationErrorType;

    fn try_from(value: BookModel) -> Result<Self, Self::Error> {
        let id = value.id.ok_or(ValidationErrorType::Id {
            message: "BookModel must have an id".to_string()
        })?;
        let isbn = ISBN::new(value.isbn.as_str())
            .map_err(|_| ValidationErrorType::Isbn {
                message: format!("BookModel must have a valid isbn: {}", value.isbn)
            })?;

        let title = RblStringVvo::try_from(value.title)
            .map_err(|e| ValidationErrorType::Title {
                message: format!("BookModel must have a valid title: {} Book ISBN: {}",
                                 e.message,
                                 value.isbn)
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
                Err(e) => Err(ValidationErrorType::Author {
                    message: format!("BookModel must have a valid list of authors: {} Book ISBN: {}",
                                     e.message,
                                     value.isbn)
                })?
            }
        }

        let publisher = Publisher::try_from(value.publisher)
            .map_err(|e| ValidationErrorType::Publisher {
                message: format!("BookModel must have a valid publisher: {} Book ISBN: {}",
                                 e.message.as_str(),
                                 value.isbn.as_str())
            })?;

        let short_description = RblStringVvo::try_from(value.short_description)
            .map_err(|e| ValidationErrorType::ShortDescription {
                message: format!("BookModel must have a valid short_description: {} Book ISBN: {}",
                                 e.message,
                                 value.isbn)
            })?;
        let long_description = RblStringVvo::try_from(value.long_description)
            .map_err(|e| ValidationErrorType::LongDescription {
                message: format!("BookModel must have a valid long_description: {} Book ISBN: {}",
                                 e.message,
                                 value.isbn)
            })?;

        let price = PositiveFloatVvo::try_from(value.price)
            .map_err(|e| ValidationErrorType::Price {
                message: format!("BookModel must have a valid price: {} Book ISBN: {}",
                                 e.message,
                                 value.isbn)
            })?;
        let year = ReleaseYearVvo::try_from(value.year)
            .map_err(|e| ValidationErrorType::Year {
                message: format!("BookModel must have a valid year: {} Book ISBN: {}",
                                 e.message,
                                 value.isbn)
            })?;
        let num_pages = PositiveIntVvo::try_from(value.num_pages)
            .map_err(|e| ValidationErrorType::NumPages {
                message: format!("BookModel must have a valid num_pages: {} Book ISBN: {}",
                                 e.message,
                                 value.isbn)
            })?;

        let genre: Genre = value.genre.into();
        let cover_image = match Asset::get(
            &value.cover_image.unwrap_or_default()
        ) {
            Some(asset) => Some(asset.data.to_vec()),
            None => match Asset::get("default.jpg") {
                Some(default_asset) => Some(default_asset.data.to_vec()),
                None => None
            }
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
                .map(AuthorDto::from)
                .collect(),
            genre: value.genre.into(),
            short_description: value.short_description.value(),
            price: value.price.value(),
            cover_image: value.cover_image.unwrap_or_default(),
            release_year: value.year.value(),
        }
    }
}

#[cfg(test)]
mod book_entity_unit_tests {
    use chrono::Datelike;
    use mongodb::bson::oid::ObjectId;
    use crate::domain::book::book_entity::Book;
    use crate::domain::core::errors::ValidationErrorType;
    use crate::infrastructure::book::book_model::{AuthorModel, BookModel, GenreModel, PublisherModel};

    #[test]
    fn when_trying_to_create_a_book_from_a_book_model_with_an_invalid_isbn_then_an_error_is_returned() {
        // Arrange
        let book_model = BookModel {
            id: Some(ObjectId::new()),
            isbn: "123456789".to_string(),
            title: "The Book of the Book".to_string(),
            authors: None,
            publisher: PublisherModel {
                name: "The Publisher".to_string(),
                address: "The Address".to_string(),
            },
            short_description: "The short description".to_string(),
            long_description: "The long description is such a long description to write".to_string(),
            price: 10.0,
            genre: GenreModel::Fiction,
            year: 2020,
            num_pages: 100,
            cover_image: None,
        };

        // Act
        let result = Book::try_from(book_model);

        // Assert
        assert!(result.is_err());
        let isbn_error = ValidationErrorType::Isbn { message: "".to_string() };
        assert_eq!(result.err().unwrap().name(), isbn_error.name());
    }

    #[test]
    fn when_trying_to_create_a_book_from_a_book_model_with_an_invalid_title_then_an_error_is_returned() {
        // Arrange
        let book_model = BookModel {
            id: Some(ObjectId::new()),
            isbn: "978-3-16-148410-0".to_string(),
            title: "The Book of the Book of the Book of the Book of the Book".to_string(),
            authors: None,
            publisher: PublisherModel {
                name: "The Publisher".to_string(),
                address: "The Address".to_string(),
            },
            short_description: "The short description".to_string(),
            long_description: "The long description is such a long description to write".to_string(),
            price: 10.0,
            genre: GenreModel::Fiction,
            year: 2020,
            num_pages: 100,
            cover_image: None,
        };

        // Act
        let result = Book::try_from(book_model);

        // Assert
        assert!(result.is_err());
        let title_error = ValidationErrorType::Title { message: "".to_string() };
        assert_eq!(result.err().unwrap().name(), title_error.name());
    }

    #[test]
    fn when_trying_to_create_a_book_from_a_book_model_with_an_invalid_list_of_authors_then_an_error_is_returned() {
        // Arrange
        let book_model = BookModel {
            id: Some(ObjectId::new()),
            isbn: "978-3-16-148410-0".to_string(),
            title: "The Book of the Book".to_string(),
            authors: Some(vec![
                AuthorModel {
                    first_name: "".to_string(),
                    last_name: "The Last Name".to_string(),
                }
            ]),
            publisher: PublisherModel {
                name: "The Publisher".to_string(),
                address: "The Address".to_string(),
            },
            short_description: "The short description".to_string(),
            long_description: "The long description is such a long description to write".to_string(),
            price: 10.0,
            genre: GenreModel::Fiction,
            year: 2020,
            num_pages: 100,
            cover_image: None,
        };

        // Act
        let result = Book::try_from(book_model);

        // Assert
        assert!(result.is_err());
        let author_error = ValidationErrorType::Author { message: "".to_string() };
        assert_eq!(result.err().unwrap().name(), author_error.name());
    }

    #[test]
    fn when_trying_to_create_a_book_from_a_book_model_with_an_invalid_publisher_name_then_an_error_is_returned() {
        // Arrange
        let book_model = BookModel {
            id: Some(ObjectId::new()),
            isbn: "978-3-16-148410-0".to_string(),
            title: "The Book of the Book".to_string(),
            authors: None,
            publisher: PublisherModel {
                name: "".to_string(),
                address: "The Address".to_string(),
            },
            short_description: "The short description".to_string(),
            long_description: "The long description is such a long description to write".to_string(),
            price: 10.0,
            genre: GenreModel::Fiction,
            year: 2020,
            num_pages: 100,
            cover_image: None,
        };

        // Act
        let result = Book::try_from(book_model);

        // Assert
        assert!(result.is_err());
        let publisher_error = ValidationErrorType::Publisher { message: "".to_string() };
        assert_eq!(result.err().unwrap().name(), publisher_error.name());
    }

    #[test]
    fn when_trying_to_create_a_book_from_a_book_model_with_an_invalid_short_description_then_an_error_is_returned() {
        // Arrange
        let book_model = BookModel {
            id: Some(ObjectId::new()),
            isbn: "978-3-16-148410-0".to_string(),
            title: "The Book of the Book".to_string(),
            authors: None,
            publisher: PublisherModel {
                name: "The Publisher".to_string(),
                address: "The Address".to_string(),
            },
            short_description: "".to_string(),
            long_description: "The long description is such a long description to write".to_string(),
            price: 10.0,
            genre: GenreModel::Fiction,
            year: 2020,
            num_pages: 100,
            cover_image: None,
        };

        // Act
        let result = Book::try_from(book_model);

        // Assert
        assert!(result.is_err());
        let short_description_error = ValidationErrorType::ShortDescription { message: "".to_string() };
        assert_eq!(result.err().unwrap().name(), short_description_error.name());
    }

    #[test]
    fn when_trying_to_create_a_book_from_a_book_model_with_an_invalid_long_description_then_an_error_is_returned() {
        // Arrange
        let book_model = BookModel {
            id: Some(ObjectId::new()),
            isbn: "978-3-16-148410-0".to_string(),
            title: "The Book of the Book".to_string(),
            authors: None,
            publisher: PublisherModel {
                name: "The Publisher".to_string(),
                address: "The Address".to_string(),
            },
            short_description: "The short description".to_string(),
            long_description: "".to_string(),
            price: 10.0,
            genre: GenreModel::Fiction,
            year: 2020,
            num_pages: 100,
            cover_image: None,
        };

        // Act
        let result = Book::try_from(book_model);

        // Assert
        assert!(result.is_err());
        let long_description_error = ValidationErrorType::LongDescription { message: "".to_string() };
        assert_eq!(result.err().unwrap().name(), long_description_error.name());
    }

    #[test]
    fn when_trying_to_create_a_book_from_a_book_model_with_an_invalid_price_then_an_error_is_returned() {
        // Arrange
        let book_model = BookModel {
            id: Some(ObjectId::new()),
            isbn: "978-3-16-148410-0".to_string(),
            title: "The Book of the Book".to_string(),
            authors: None,
            publisher: PublisherModel {
                name: "The Publisher".to_string(),
                address: "The Address".to_string(),
            },
            short_description: "The short description".to_string(),
            long_description: "The long description is such a long description to write".to_string(),
            price: -10.0,
            genre: GenreModel::Fiction,
            year: 2020,
            num_pages: 100,
            cover_image: None,
        };

        // Act
        let result = Book::try_from(book_model);

        // Assert
        assert!(result.is_err());
        let price_error = ValidationErrorType::Price { message: "".to_string() };
        assert_eq!(result.err().unwrap().name(), price_error.name());
    }

    #[test]
    fn when_trying_to_create_a_book_from_a_book_model_with_an_invalid_number_of_pages_then_an_error_is_returned() {
        // Arrange
        let book_model = BookModel {
            id: Some(ObjectId::new()),
            isbn: "978-3-16-148410-0".to_string(),
            title: "The Book of the Book".to_string(),
            authors: None,
            publisher: PublisherModel {
                name: "The Publisher".to_string(),
                address: "The Address".to_string(),
            },
            short_description: "The short description".to_string(),
            long_description: "The long description is such a long description to write".to_string(),
            price: 10.0,
            genre: GenreModel::Fiction,
            year: 2020,
            num_pages: 0,
            cover_image: None,
        };

        // Act
        let result = Book::try_from(book_model);

        // Assert
        assert!(result.is_err());
        let num_pages_error = ValidationErrorType::NumPages { message: "".to_string() };
        assert_eq!(result.err().unwrap().name(), num_pages_error.name());
    }

    #[test]
    fn when_trying_to_create_a_book_from_a_book_model_with_an_invalid_year_then_an_error_is_returned() {
        // Arrange
        let book_model = BookModel {
            id: Some(ObjectId::new()),
            isbn: "978-3-16-148410-0".to_string(),
            title: "The Book of the Book".to_string(),
            authors: None,
            publisher: PublisherModel {
                name: "The Publisher".to_string(),
                address: "The Address".to_string(),
            },
            short_description: "The short description".to_string(),
            long_description: "The long description is such a long description to write".to_string(),
            price: 10.0,
            genre: GenreModel::Fiction,
            year: chrono::Utc::now().year() as u32 + 1,
            num_pages: 100,
            cover_image: None,
        };

        // Act
        let result = Book::try_from(book_model);

        // Assert
        assert!(result.is_err());
        let year_error = ValidationErrorType::Year { message: "".to_string() };
        assert_eq!(result.err().unwrap().name(), year_error.name());
    }

    #[test]
    fn when_trying_to_create_a_book_from_a_valid_book_model_then_a_book_is_returned() {
        // Arrange
        let book_model = BookModel {
            id: Some(ObjectId::new()),
            isbn: "978-3-16-148410-0".to_string(),
            title: "The Book of the Book".to_string(),
            authors: None,
            publisher: PublisherModel {
                name: "The Publisher".to_string(),
                address: "The Address".to_string(),
            },
            short_description: "The short description".to_string(),
            long_description: "The long description is such a long description to write".to_string(),
            price: 10.0,
            genre: GenreModel::Fiction,
            year: 2020,
            num_pages: 100,
            cover_image: None,
        };

        // Act
        let result = Book::try_from(book_model);

        // Assert
        assert!(result.is_ok());
    }
}