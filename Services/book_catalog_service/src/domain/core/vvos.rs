use std::fmt;
use std::fmt::{Display, Formatter};
use domain_patterns::models::ValueObject;
use chrono::Datelike;
use crate::domain::core::errors::ValidationError;

/// Range based length string vvo type
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

/// Positive integer vvo type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositiveIntVvo {
    value: u32,
}

impl TryFrom<u32> for PositiveIntVvo {
    type Error = ValidationError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(PositiveIntVvo { value })
    }
}

impl ValueObject<u32> for PositiveIntVvo {
    type ValueError = ValidationError;

    fn validate(value: &u32) -> Result<(), Self::ValueError> {
        if value <= &0 {
            return Err(ValidationError {
                message: "Value must be greater than 0".to_string()
            });
        }

        Ok(())
    }

    fn value(&self) -> u32 {
        self.value
    }
}

impl Display for PositiveIntVvo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Positive float vvo type
#[derive(Debug, Clone, PartialEq)]
pub struct PositiveFloatVvo {
    value: f64,
}

impl TryFrom<f64> for PositiveFloatVvo {
    type Error = ValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(PositiveFloatVvo { value })
    }
}

impl ValueObject<f64> for PositiveFloatVvo {
    type ValueError = ValidationError;

    fn validate(value: &f64) -> Result<(), Self::ValueError> {
        if value <= &0.0 {
            return Err(ValidationError {
                message: "Value must be greater than 0".to_string()
            });
        }

        Ok(())
    }

    fn value(&self) -> f64 {
        self.value
    }
}

impl Display for PositiveFloatVvo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Release year vvo type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseYearVvo<const START_YEAR: usize> {
    value: u32,
}

impl<const START_YEAR: usize> TryFrom<u32> for ReleaseYearVvo<START_YEAR> {
    type Error = ValidationError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(ReleaseYearVvo { value })
    }
}

impl<const START_YEAR: usize> ValueObject<u32> for ReleaseYearVvo<START_YEAR> {
    type ValueError = ValidationError;

    fn validate(value: &u32) -> Result<(), Self::ValueError> {
        let current_year = chrono::Utc::now().year() as u32;
        if value < &(START_YEAR as u32) || value > &current_year {
            return Err(ValidationError {
                message: format!("Value must be between {} and {}", START_YEAR, current_year)
            });
        }

        Ok(())
    }

    fn value(&self) -> u32 {
        self.value
    }
}

impl<const START_YEAR: usize> Display for ReleaseYearVvo<START_YEAR> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}