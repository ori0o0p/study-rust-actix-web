use std::collections::HashMap;
use std::sync::{Mutex};
use lazy_static::lazy_static;
use crate::model::{Article, UploadArticleDto};

lazy_static! {
    static ref ARTICLES: Mutex<HashMap<u32, Article>> = Mutex::new(HashMap::new());
}

pub async fn get_article_default(id: u32) -> Article {
    let articles = ARTICLES.lock().unwrap();

    match articles.get(&id) {
        Some(article) => article.clone(),
        None => Article {
            id: 1,
            title: "".parse().unwrap(),
            content: "".parse().unwrap(),
        }
    }
}

static INDEX: Mutex<u32> = Mutex::new(0);

pub async fn upload_article(request: UploadArticleDto) {
    let mut index = INDEX.lock().unwrap();
    *index += 1;

    let article = Article {
        id: *index,
        title: request.title,
        content: request.content,
    };

    let mut articles = ARTICLES.lock().unwrap();
    articles.insert(article.id, article);
}


pub async fn find_all() -> Vec<Article> {
    let articles = ARTICLES.lock().unwrap();

    return articles.values()
        .cloned()
        .collect()
}