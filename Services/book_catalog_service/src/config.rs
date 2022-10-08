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
