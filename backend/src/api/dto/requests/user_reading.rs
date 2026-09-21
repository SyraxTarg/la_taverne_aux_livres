use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Clone, ToSchema)]
pub struct UserReadingDto {
    pub book_id: String,
    pub note: Option<i32>,
}
