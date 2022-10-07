use std::fmt;
use std::fmt::{Display, Formatter};
use domain_patterns::models::ValueObject;
use crate::domain::core::errors::ValidationError;

/// Range based length alphanumeric string vvo type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RblStringVvo<const MIN_LENGTH: usize, const MAX_LENGTH: usize> {
    value: String,
}

impl<const MIN_LENGTH: usize, const MAX_LENGTH: usize> TryFrom<String>
for RblStringVvo<MIN_LENGTH, MAX_LENGTH> {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(RblStringVvo { value })
    }
}

impl<const MIN_LENGTH: usize, const MAX_LENGTH: usize> ValueObject<String>
for RblStringVvo<MIN_LENGTH, MAX_LENGTH> {
    type ValueError = ValidationError;

    fn validate(value: &String) -> Result<(), Self::ValueError> {
        if value.len() < MIN_LENGTH || value.len() > MAX_LENGTH {
            return Err(ValidationError {
                message: format!("Length must be between {} and {} characters long", MIN_LENGTH, MAX_LENGTH)
            });
        }

        Ok(())
    }

    fn value(&self) -> String {
        self.value.clone()
    }
}

impl<const MIN_LENGTH: usize, const MAX_LENGTH: usize> Display
for RblStringVvo<MIN_LENGTH, MAX_LENGTH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.as_str())
    }
}
