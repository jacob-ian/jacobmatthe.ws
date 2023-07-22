use actix_web::{web, Responder};

use crate::{components::article, html::HtmlResponse};

pub async fn post(path: web::Path<String>) -> impl Responder {
    let stub = path.into_inner();
    return HtmlResponse::builder()
        .title("Post | Jacob Matthews")
        .description("This is a post")
        .body(article::new(
            format!(
                r#"
            <h1>{}</h1>
            <p>This is some content</p>
            "#,
                stub
            )
            .to_string(),
        ))
        .build();
}
