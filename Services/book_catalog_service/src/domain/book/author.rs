use domain_patterns::models::ValueObject;
use crate::domain::core::errors::ValidationError;
use crate::domain::core::vvos::RblStringVvo;
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

impl From<Author> for crate::grpc::Author {
    fn from(value: Author) -> Self {
        Self {
            first_name: value.first_name.value(),
            last_name: value.last_name.value(),
        }
    }
}