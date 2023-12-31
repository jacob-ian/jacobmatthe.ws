use actix_web::web;

use crate::{
    cms::{posts::Post, Client},
    components::article,
    errors::Error,
    html::HtmlResponse,
};

async fn get_post_by_stub(client: &Client, stub: &str) -> Result<Post, Error> {
    return client
        .get()
        .path(format!("posts/{}", stub))
        .json::<Post>()
        .await;
}

pub async fn post(
    client: web::Data<Client>,
    stub: web::Path<String>,
) -> Result<HtmlResponse, Error> {
    let post = get_post_by_stub(&client, &stub).await?;

    return Ok(HtmlResponse::builder()
        .title(format!("{} | Jacob Matthews", post.title))
        .description(post.description)
        .body(
            article::builder()
                .title(post.title)
                .published_at(Some(post.published_at))
                .content(post.content)
                .render(),
        )
        .build());
}
