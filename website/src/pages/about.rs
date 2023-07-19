use actix_web::Responder;

use crate::{components::article, html::HtmlResponse};

pub async fn about() -> impl Responder {
    return HtmlResponse::builder()
        .title("About | jacobmatthe.ws")
        .body(article::new(
            r#"
        <h1>About</h1>
        "#
            .to_string(),
        ))
        .build();
}
