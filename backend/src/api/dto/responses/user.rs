use serde::{Serialize, Deserialize};
use serde_json::Number;
use utoipa::ToSchema;
use crate::api::dto::responses::role::RoleResponseDto;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserResponseDto {
    #[schema(value_type = u64)]
    pub id: Number,
    pub email: String,
    pub role: RoleResponseDto
}

