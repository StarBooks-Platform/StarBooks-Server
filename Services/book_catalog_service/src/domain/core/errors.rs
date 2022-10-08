use std::error;
use std::fmt::{Display, Formatter, Result};
use derive_more::{Display, Error};

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message.as_str())
    }
}

impl error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, Display, Error)]
pub enum ValidationErrorType {
    #[display(fmt = message)]
    Id {
        message: String,
    },
    #[display(fmt = message)]
    Isbn {
        message: String,
    },
    #[display(fmt = message)]
    Title {
        message: String,
    },
    #[display(fmt = message)]
    Author {
        message: String,
    },
    #[display(fmt = message)]
    ShortDescription {
        message: String,
    },
    #[display(fmt = message)]
    LongDescription {
        message: String,
    },
    #[display(fmt = message)]
    Price {
        message: String,
    },
    #[display(fmt = message)]
    NumPages {
        message: String,
    },
    #[display(fmt = message)]
    Year {
        message: String,
    },
    #[display(fmt = message)]
    Publisher {
        message: String,
    }
}

impl ValidationErrorType {
    pub fn name(&self) -> String {
        match self {
            ValidationErrorType::Id { .. } => "IdValidationError".to_string(),
            ValidationErrorType::Isbn { .. } => "IsbnValidationError".to_string(),
            ValidationErrorType::Title { .. } => "TitleValidationError".to_string(),
            ValidationErrorType::Author { .. } => "AuthorValidationError".to_string(),
            ValidationErrorType::ShortDescription { .. } => "ShortDescriptionValidationError".to_string(),
            ValidationErrorType::LongDescription { .. } => "LongDescriptionValidationError".to_string(),
            ValidationErrorType::Price { .. } => "PriceValidationError".to_string(),
            ValidationErrorType::NumPages { .. } => "PagesValidationError".to_string(),
            ValidationErrorType::Year { .. } => "YearValidationError".to_string(),
            ValidationErrorType::Publisher { .. } => "PublisherValidationError".to_string(),
        }
    }
}