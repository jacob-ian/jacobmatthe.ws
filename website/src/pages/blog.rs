use actix_web::web;

use crate::{
    cms::{posts::Post, Client},
    components::title,
    errors::Error,
    html::HtmlResponse,
};

async fn get_posts(client: &Client) -> Result<Vec<Post>, Error> {
    return client
        .get()
        .path(String::from("posts"))
        .json::<Vec<Post>>()
        .await;
}

pub async fn blog(client: web::Data<Client>) -> Result<HtmlResponse, Error> {
    let posts = get_posts(&client)
        .await?
        .into_iter()
        .map(|p| {
            format!(
                r#"<a href="/{stub}">> {name}</a>"#,
                stub = p.stub,
                name = p.title
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
        .to_string();

    return Ok(HtmlResponse::builder()
        .title(String::from("Blog | Jacob Matthews"))
        .body(format!(
            r#"
            {title}
            <h2 class="my-5">Latest Posts:</h2>
            {posts}
            "#,
            title = title::render("Blog"),
            posts = posts
        ))
        .build());
}
