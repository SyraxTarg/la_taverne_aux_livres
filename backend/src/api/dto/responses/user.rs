use serde::{Serialize, Deserialize};
use serde_json::Number;
use utoipa::ToSchema;
use crate::api::dto::responses::role::RoleResponseDto;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserResponseDto {
    #[schema(value_type = u64)]
    pub id: Number,
    pub username: String,
    pub email: String,
    pub role: RoleResponseDto,
    pub can_be_recommanded: bool
}



#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserSimpleResponseDto {
    #[schema(value_type = u64)]
    pub id: Number,
    pub username: String,
    pub email: String
}
