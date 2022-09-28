use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum InfrastructureError {
    #[display(fmt = message)]
    MongoDbError {
        message: String,
    },
    #[display(fmt = message)]
    NoEntityFoundError {
        message: String,
    },
    #[display(fmt = message)]
    InvalidEntityFoundError {
        message: String,
    }
}

impl InfrastructureError {
    fn name(&self) -> String {
        match self {
            InfrastructureError::MongoDbError { .. } => "MongoDbError".to_string(),
            InfrastructureError::NoEntityFoundError { .. } => "NoEntityFoundError".to_string(),
            InfrastructureError::InvalidEntityFoundError { .. } => "InvalidEntityFoundError".to_string(),
        }
    }
}