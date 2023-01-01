use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateArticle {
    pub author: String,
    pub title: String,
    pub content: String,
    pub lg_content: String,
    pub date: u64,
}
