use domain_patterns::models::ValueObject;
use crate::domain::core::errors::ValidationError;
use crate::domain::core::vvos::RblStringVvo;
use crate::grpc::AuthorDto;
use crate::infrastructure::book::book_model::AuthorModel;

#[derive(Debug)]
pub struct Author {
    pub first_name: RblStringVvo<1, 50>,
    pub last_name: RblStringVvo<1, 50>,
}

impl TryFrom<AuthorModel> for Author {
    type Error = ValidationError;

    fn try_from(value: AuthorModel) -> Result<Self, Self::Error> {
        let first_name = RblStringVvo::try_from(value.first_name)
            .map_err(|e| ValidationError {
            message: format!("AuthorModel.first_name is invalid: {}", e.message.as_str())
        })?;
        let last_name = RblStringVvo::try_from(value.last_name)
            .map_err(|e| ValidationError {
            message: format!("AuthorModel.last_name is invalid: {}", e.message.as_str())
        })?;

        Ok(Author {
            first_name,
            last_name,
        })
    }
}

impl From<Author> for AuthorDto {
    fn from(value: Author) -> Self {
        Self {
            first_name: value.first_name.value(),
            last_name: value.last_name.value(),
        }
    }
}

#[cfg(test)]
mod author_entity_unit_tests {
    use domain_patterns::models::ValueObject;
    use crate::domain::book::author::Author;
    use crate::infrastructure::book::book_model::AuthorModel;

    #[test]
    fn when_trying_to_create_an_author_entity_with_invalid_first_name_then_an_error_is_returned() {
        // Arrange
        let author_model = AuthorModel {
            first_name: "".to_string(),
            last_name: "Doe".to_string(),
        };

        // Act
        let result = Author::try_from(author_model);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().message, "AuthorModel.first_name is invalid: Length must be between 1 and 50 characters long");
    }

    #[test]
    fn when_trying_to_create_an_author_entity_with_invalid_last_name_then_an_error_is_returned() {
        // Arrange
        let author_model = AuthorModel {
            first_name: "John".to_string(),
            last_name: "".to_string(),
        };

        // Act
        let result = Author::try_from(author_model);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().message, "AuthorModel.last_name is invalid: Length must be between 1 and 50 characters long");
    }

    #[test]
    fn when_trying_to_create_an_author_entity_with_valid_first_and_last_name_then_an_author_entity_is_returned() {
        // Arrange
        let author_model = AuthorModel {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };

        // Act
        let result = Author::try_from(author_model);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.as_ref().unwrap().first_name.value(), "John");
        assert_eq!(result.as_ref().unwrap().last_name.value(), "Doe");
    }
}