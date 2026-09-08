use serde_json::Number;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::api::dto::responses::author::AuthorDto;
use crate::api::dto::responses::categories::CategoryDto;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct BookDto {
    pub id: String,
    pub title: String,
    pub authors: Vec<AuthorDto>,
    pub description: Option<String>,
    pub image_url: Option<String>,

    #[schema(value_type = u64)]
    pub page_count: Number,

    pub categories: Vec<CategoryDto>,

    #[schema(value_type = u64)]
    pub average_rating: Number,

    #[schema(value_type = u64)]
    pub ratings_count: Number,
    pub maturity_rating: String,
    pub published_date: String
}