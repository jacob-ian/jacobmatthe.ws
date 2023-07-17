use actix_web::{web, App, HttpServer, Responder};
use website::html::HtmlResponse;

async fn page() -> impl Responder {
    return HtmlResponse::builder()
        .title("How to page a page")
        .body("<p>Hi there</p>")
        .build();
}

async fn index() -> impl Responder {
    return HtmlResponse::builder()
        .title("Welcome to Jacob's Blog")
        .body(
            r#"
        <h1 class="text-5xl font-bold">Welcome to Jacob's Blog</h1>
        <h2>A place to read things</h2>
        <p>There is some content here.</p>
        <p>Have I considered the generation of nav bar items?</p>
        <p>I don't think I need it </p>
        <code>async fn index() -> impl Responder { HttpResponse::Ok().into() }</code>
        "#,
        )
        .build();
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
