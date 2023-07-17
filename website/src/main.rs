use actix_web::{App, HttpServer};
use website::pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || App::new().configure(pages::config))
        .bind("0.0.0.0:4000")?
        .run();

    println!("\n-------\n\nSuccessfully started jacobmatthe.ws!\n");
    println!("Listening on {}:{}\n\n-------", "0.0.0.0", 4000);

    return server.await;
}
