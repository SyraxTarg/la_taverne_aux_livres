use serde::Serialize;
use serde_json::Number;
use crate::api::dto::responses::author::AuthorDto; 
use crate::api::dto::responses::categories::CategoryDto; 

#[derive(Serialize)]
pub struct BookDto {
    pub id: String,
    pub title: String,
    pub authors: Vec<AuthorDto>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub page_count: Number,
    pub categories: Vec<CategoryDto>,
    pub average_rating: Number,
    pub ratings_count: Number,
    pub maturity_rating: String,
    pub published_date: String
}