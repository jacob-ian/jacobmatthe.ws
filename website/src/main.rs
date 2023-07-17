use std::process;

use actix_web::{App, HttpServer};
use website::{config::Config, pages};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match Config::from_toml() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Could not read config: {}", e);
            process::exit(1)
        }
    };
    let server = HttpServer::new(move || App::new().configure(pages::config))
        .bind((config.host.clone(), config.port))?
        .run();

    println!("\n-------\n\nSuccessfully started jacobmatthe.ws!\n");
    println!("Listening on {}:{}\n\n-------", config.host, config.port);

    return server.await;
}
