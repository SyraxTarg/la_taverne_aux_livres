use serde::{Serialize, Deserialize};
use serde_json::Number;
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema)]
pub struct RoleResponseDto {
    #[schema(value_type = u64)]
    pub id: Number,
    pub role: String
}