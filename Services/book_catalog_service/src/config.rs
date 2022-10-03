use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GrpcConfiguration {
    host: String,
    port: u16,
}

impl GrpcConfiguration {
    pub fn uri(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize, Debug)]
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
    pub fn uri(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
