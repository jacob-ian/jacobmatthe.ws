use actix_web::Responder;

use crate::{components, html::HtmlResponse};

pub async fn home() -> impl Responder {
    return HtmlResponse::builder()
        .title("Welcome to Jacob's Blog")
        .body(format!(
            r#"
        <h1 class="text-4xl font-extrabold mb-10 text-sky-500">Welcome to Jacob's Blog</h1>
        <p>There is some content here.</p>
        <p>This is a code block</p>
        {code}
        "#,
            code = components::code::new(
                r#"
                pub async fn hi() -> &'static str {
                    return "hi";
                }
                "#
            ),
        ))
        .build();
}
