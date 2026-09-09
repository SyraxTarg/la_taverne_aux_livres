use serde::{Deserialize};
use utoipa::ToSchema;

#[derive(Deserialize, Clone, ToSchema)]
pub struct CreateCommentDto {
    pub content: String,
    pub book_id: String,
    pub parent_id: Option<i32>,
}