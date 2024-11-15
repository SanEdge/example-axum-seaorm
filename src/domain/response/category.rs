use crate::entities::categories;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct CategoryResponse {
    pub id: i32,
    pub name: String,
}

impl From<categories::Model> for CategoryResponse {
    fn from(category: categories::Model) -> Self {
        CategoryResponse {
            id: category.id,
            name: category.name,
        }
    }
}