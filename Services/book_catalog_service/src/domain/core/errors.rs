use std::error;
use std::fmt::{Display, Formatter, Result};

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
