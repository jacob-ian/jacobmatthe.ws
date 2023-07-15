use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};

async fn page() -> impl Responder {
    return HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<html><body><nav><a href=\"/\">Home</a></nav><h1>Page</h1><img src=\"/uploads/test.jpg\"/></body></html>");
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
