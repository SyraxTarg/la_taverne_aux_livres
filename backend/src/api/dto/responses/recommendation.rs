use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::api::dto::responses::book::BookDto;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RecommendedBookDto {
    #[serde(flatten)]
    pub book: BookDto,
    pub similarity_score: f32,
}

