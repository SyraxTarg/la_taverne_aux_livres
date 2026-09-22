use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct BookRatingsStatsDto {
    pub book_id: String,
    pub average_rating: Option<f64>,
    pub ratings_count: u64,
}

