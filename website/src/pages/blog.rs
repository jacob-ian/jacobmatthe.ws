use actix_web::web;

use crate::{
    cms::Client,
    components::{all_posts, latest_posts, title},
    errors::Error,
    html::HtmlResponse,
};

pub async fn blog(client: web::Data<Client>) -> Result<HtmlResponse, Error> {
    return Ok(HtmlResponse::builder()
        .title(String::from("Blog | Jacob Matthews"))
        .body(format!(
            r#"
            {title}
            {latest}
            {all}
            "#,
            title = title::render("Blog"),
            latest = latest_posts::render(&client).await?,
            all = all_posts::render(&client).await?
        ))
        .build());
}
