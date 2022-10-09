use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GrpcConfiguration {
    host: String,
    port: u16,
    pub buffer_size: u32,
}

impl GrpcConfiguration {
    pub fn new(host: String, port: u16, buffer_size: u32) -> Self {
        GrpcConfiguration {
            host,
            port,
            buffer_size,
        }
    }

    pub fn uri(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for GrpcConfiguration {
    fn default() -> Self {
        GrpcConfiguration {
            host: "localhost".to_string(),
            port: 50051,
            buffer_size: 100,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct MongoConfiguration {
    host: String,
    port: u16,
    #[serde(rename = "db")]
    pub database: String,
    pub collection: String,
    #[serde(rename = "user")]
    username: String,
    #[serde(rename = "pass")]
    password: String,
}

impl MongoConfiguration {
    pub fn new(host: String, port: u16, database: String, collection: String, username: String, password: String) -> Self {
        MongoConfiguration {
            host,
            port,
            database,
            collection,
            username,
            password,
        }
    }

    pub fn uri(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

impl Default for MongoConfiguration {
    fn default() -> Self {
        MongoConfiguration {
            host: "localhost".to_string(),
            port: 27017,
            database: "catalog_db".to_string(),
            collection: "book".to_string(),
            username: "sami".to_string(),
            password: "root2002".to_string(),
        }
    }
}
