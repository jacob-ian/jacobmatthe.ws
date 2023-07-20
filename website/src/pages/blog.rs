use actix_web::Responder;

use crate::{components::article, html::HtmlResponse};

pub async fn blog() -> impl Responder {
    return HtmlResponse::builder()
        .title("Blog | Jacob Matthews")
        .body(article::new(
            r#"
        <h1>Blog</h1>
        <p>Posts go here</p>
    "#
            .to_string(),
        ))
        .build();
}
