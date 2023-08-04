use actix_web::web;

use crate::{
    cms::{posts::Post, Client},
    components::{latest_posts, title},
    errors::Error,
    html::HtmlResponse,
};

async fn get_all_posts(client: &Client) -> Result<Vec<Post>, Error> {
    return client
        .get()
        .path(String::from("posts"))
        .json::<Vec<Post>>()
        .await;
}

pub async fn blog(client: web::Data<Client>) -> Result<HtmlResponse, Error> {
    return Ok(HtmlResponse::builder()
        .title(String::from("Blog | Jacob Matthews"))
        .body(format!(
            r#"
            {title}
            {latest}
            <h2 class="my-5 text-lg">All Posts:</h2>
            {all}
            "#,
            title = title::render("Blog"),
            latest = latest_posts::render(&client).await?,
            all = "hi"
        ))
        .build());
}
