use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct UploadArticleDto {
    pub title: String,
    pub content: String,
}