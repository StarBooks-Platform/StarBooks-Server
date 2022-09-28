use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use rust_embed::RustEmbed;

#[derive(Debug, Serialize, Deserialize)]
pub struct BookModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub isbn: String,
    pub title: String,
    pub publisher: PublisherModel,
    pub authors: Option<Vec<AuthorModel>>,
    pub short_description: String,
    pub long_description: String,
    pub year: u32,
    pub num_pages: u32,
    #[serde(rename = "image_file")]
    pub cover_image: Option<String>,
    pub price: f32,
    pub genre: GenreModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublisherModel {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorModel {
    #[serde(rename = "firstname")]
    pub first_name: String,
    #[serde(rename = "lastname")]
    pub last_name: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum GenreModel {
    Unknown = 0,
    ScienceFiction = 1,
    Science = 2,
    Fiction = 3,
    NonFiction = 4,
}

#[derive(RustEmbed)]
#[folder = "assets/images/"]
pub struct Asset;