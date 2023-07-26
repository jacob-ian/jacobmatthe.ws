use chrono::{DateTime, Utc};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Post {
    pub id: String,
    pub author_id: String,
    pub stub: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: DateTime<Utc>,
}
