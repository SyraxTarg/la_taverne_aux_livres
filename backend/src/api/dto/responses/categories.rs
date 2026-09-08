use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema)]
pub struct CategoryDto {
    pub name: String
}