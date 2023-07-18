use actix_web::Responder;

use crate::html::HtmlResponse;

pub async fn home() -> impl Responder {
    return HtmlResponse::builder()
        .title("Welcome to Jacob's Blog")
        .body(
            r#"
        <h1 class="text-5xl font-medium mb-10 text-sky-500">Welcome to Jacob's Blog</h1>
        <p>There is some content here.</p>
        "#,
        )
        .build();
}
