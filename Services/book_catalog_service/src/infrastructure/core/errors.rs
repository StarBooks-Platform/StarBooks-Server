use derive_more::{Display, Error};

//TODO: add more error types
#[derive(Debug, Display, Error)]
pub enum InfrastructureError {
    #[display(fmt = message)]
    MongoDb {
        message: String,
    },
    #[display(fmt = message)]
    NoEntityFound {
        message: String,
    },
    #[display(fmt = message)]
    InvalidEntityFound {
        message: String,
    }
}

impl InfrastructureError {
    pub fn name(&self) -> String {
        match self {
            InfrastructureError::MongoDb { .. } => "MongoDbError".to_string(),
            InfrastructureError::NoEntityFound { .. } => "NoEntityFoundError".to_string(),
            InfrastructureError::InvalidEntityFound { .. } => "InvalidEntityFoundError".to_string(),
        }
    }
}