use actix_web::web;

use crate::{
    cms::{posts::Post, Client},
    components::article,
    errors::Error,
    html::HtmlResponse,
};

async fn get_about_page(client: &Client) -> Result<Post, Error> {
    return client
        .get()
        .path(String::from("posts/about"))
        .json::<Post>()
        .await;
}

pub async fn about(client: web::Data<Client>) -> Result<HtmlResponse, Error> {
    let about = get_about_page(&client).await?;
    return Ok(HtmlResponse::builder()
        .title(String::from("About | Jacob Matthews"))
        .description(about.description)
        .body(
            article::builder()
                .title(about.title)
                .content(about.content)
                .render(),
        )
        .build());
}
