use serde::Deserialize;

use crate::errors::Error;

use super::Client;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: String,
    pub stub: String,
    pub title: String,
}

pub async fn get_post_by_stub(client: &Client, stub: &str) -> Result<Post, Error> {
    return client
        .get()
        .path(format!("posts/{}", stub))
        .json::<Post>()
        .await;
}
