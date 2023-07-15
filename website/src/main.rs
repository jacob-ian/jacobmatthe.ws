use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    return HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<html><body><h1>Yes!</h1></body></html>");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || App::new().route("/", web::get().to(index)))
        .bind("0.0.0.0:4000")?
        .run();

    println!("\n-------\n\nSuccessfully started jacobmatthe.ws!\n");
    println!("Listening on {}:{}\n\n-------", "0.0.0.0", 4000);

    return server.await;
}
