use sqlx::SqlitePool;

use crate::model::contact::Contact;

pub struct DataBase {
    pub url: String,
    pub db: SqlitePool,
}

impl DataBase {
    pub async fn new(url: String) -> Self {
        Self {
            url: url.clone(),
            db: SqlitePool::connect(&url).await.unwrap(),
        }
    }

    pub async fn save(contact: Contact)  {
    }
}
