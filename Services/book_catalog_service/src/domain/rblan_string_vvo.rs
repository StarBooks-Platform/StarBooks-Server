use std::{error, fmt};
use std::fmt::{Display, Formatter};
use domain_patterns::models::ValueObject;

/// Range based length alphanumeric string vvo type
#[derive(Clone, PartialEq, Eq)]
pub struct RblanStringVVO<const MIN_LENGTH: usize, const MAX_LENGTH: usize> {
    value: String,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

impl error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl<const MIN_LENGTH: usize, const MAX_LENGTH: usize> TryFrom<String>
for RblanStringVVO<MIN_LENGTH, MAX_LENGTH> {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(RblanStringVVO { value })
    }
}

impl<const MIN_LENGTH: usize, const MAX_LENGTH: usize> ValueObject<String>
for RblanStringVVO<MIN_LENGTH, MAX_LENGTH> {
    type ValueError = ValidationError;

    fn validate(value: &String) -> Result<(), Self::ValueError> {
        if value.len() < MIN_LENGTH || value.len() > MAX_LENGTH {
            return Err(ValidationError {
                message: format!("Length must be between {} and {}", MIN_LENGTH, MAX_LENGTH)
            });
        }

        if !value.chars().all(|c| c.is_alphanumeric() || c == ' ') {
            return Err(ValidationError {
                message: "Must be alphanumeric or space".to_string()
            });
        }

        Ok(())
    }

    fn value(&self) -> String {
        self.value.clone()
    }
}

impl<const MIN_LENGTH: usize, const MAX_LENGTH: usize> Display
for RblanStringVVO<MIN_LENGTH, MAX_LENGTH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.as_str())
    }
}
