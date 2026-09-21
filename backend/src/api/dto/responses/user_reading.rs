use serde::{Deserialize, Serialize};
use utoipa::ToSchema;




#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserReadingResponseDto {
    pub id: i32,
    pub user_id: i32,
    pub book_id: String,
    pub note: Option<i32>,
    pub read_at: String,
}
