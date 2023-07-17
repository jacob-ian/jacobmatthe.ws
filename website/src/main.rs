use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use website::html::HtmlResponse;

async fn page() -> impl Responder {
    return HtmlResponse::builder()
        .title(String::from("Page | jacobmatthe.ws"))
        .body(
            r#"
        <h1>This is a page</h1>
        <p>This is some content</p>
            "#
            .to_string(),
        )
        .build();
}

async fn index() -> impl Responder {
    return HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<html><body><h1>Welcome!</h1><a href=\"/page\">Page</a></body></html>");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/page", web::get().to(page))
    })
    .bind("0.0.0.0:4000")?
    .run();

    println!("\n-------\n\nSuccessfully started jacobmatthe.ws!\n");
    println!("Listening on {}:{}\n\n-------", "0.0.0.0", 4000);

    return server.await;
}
