use serde::{Serialize};
use utoipa::ToSchema;


#[derive(Serialize, ToSchema)]
pub struct CommentResponseDto {
    pub id: i32,
    pub content: String,
    pub user: UserCommentResponseDto,
    pub book_id: String,
    pub parent_id: Option<i32>,
    pub created_at: String,
}

#[derive(Serialize, ToSchema)]
pub struct CommentWithRepliesDto {
    pub id: i32,
    pub content: String,
    pub user: UserCommentResponseDto,
    pub book_id: String,
    pub created_at: String,
    pub reponses: Vec<CommentResponseDto>,
}

#[derive(Serialize, ToSchema)]
pub struct UserCommentResponseDto {
    pub id: i32,
    pub email: String,
}