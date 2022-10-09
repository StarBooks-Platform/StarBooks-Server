use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServerErrorType {
    #[display(fmt = message)]
    MongoDb {
        message: String,
    },
    #[display(fmt = message)]
    InvalidEntityFound {
        message: String,
    }
}

impl ServerErrorType {
    pub fn name(&self) -> String {
        match self {
            ServerErrorType::MongoDb { .. } => "MongoDbError".to_string(),
            ServerErrorType::InvalidEntityFound { .. } => "InvalidEntityFoundError".to_string(),
        }
    }
}