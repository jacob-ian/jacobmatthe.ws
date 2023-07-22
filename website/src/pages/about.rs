use actix_web::Responder;

use crate::{components::article, html::HtmlResponse};

pub async fn about() -> impl Responder {
    return HtmlResponse::builder()
        .title("About | Jacob Matthews")
        .body(article::new(
            r#"
        <h1>About</h1>
        <p>blah</p>
        "#
            .to_string(),
        ))
        .build();
}
