use serde::Serialize;
// 1. On ajoute crate:: au début
// 2. On remplace le : par :: avant BookDto
// 3. On ajoute le point-virgule à la fin
use crate::api::dto::responses::book::BookDto; 

#[derive(Serialize)]
pub struct PaginationDto {
    pub total_items: usize,
    pub offset: usize,
    pub limit: usize,
}


#[derive(Serialize)]
pub struct BookSearchResponse {
    pub pagination: PaginationDto,
    pub livres: Vec<BookDto>,
}