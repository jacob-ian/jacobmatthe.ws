use actix_web::web;

use crate::{
    cms::{self, Client},
    components::article,
    errors::Error,
    html::HtmlResponse,
};

pub async fn post(
    client: web::Data<Client>,
    stub: web::Path<String>,
) -> Result<HtmlResponse, Error> {
    let post = cms::posts::get_post_by_stub(&client, &stub).await?;

    return Ok(HtmlResponse::builder()
        .title(format!("{} | Jacob Matthews", post.title))
        .description(String::from("This is a post"))
        .body(article::new(
            format!(
                r#"
            <h1>{title}</h1>
            <p>This is some content</p>
            "#,
                title = post.title
            )
            .to_string(),
        ))
        .build());
}
