use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = message)]
    MongoDb {
        message: String,
    },
    #[display(fmt = message)]
    InvalidEntityFound {
        message: String,
    }
}

impl ServerError {
    pub fn name(&self) -> String {
        match self {
            ServerError::MongoDb { .. } => "MongoDbError".to_string(),
            ServerError::InvalidEntityFound { .. } => "InvalidEntityFoundError".to_string(),
        }
    }
}