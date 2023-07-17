use actix_web::Responder;

use crate::html::HtmlResponse;

pub async fn home() -> impl Responder {
    return HtmlResponse::builder()
        .title("Welcome to Jacob's Blog")
        .body(
            r#"
        <h1 class="text-5xl font-bold font-serif">Welcome to Jacob's Blog</h1>
        <h2>A place to read things</h2>
        <p>There is some content here.</p>
        <p>Have I considered the generation of nav bar items?</p>
        <p>I don't think I need it </p>
        <code>async fn index() -> impl Responder { HttpResponse::Ok().into() }</code>
        "#,
        )
        .build();
}
