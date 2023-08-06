use std::process;

use actix_web::{web, App, HttpServer};
use website::{cms, config::Config, files, pages};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match Config::from_toml() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Could not read config: {}", e);
            process::exit(1)
        }
    };

    let app_config = web::Data::new(config.clone());
    let cms_client = match cms::Client::from_config(&config) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Could not connect to backend: {}", e);
            process::exit(1)
        }
    };
    let cms_client = web::Data::new(cms_client);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&app_config))
            .app_data(web::Data::clone(&cms_client))
            .configure(pages::config)
            .configure(files::config)
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("\n-------\n\nSuccessfully started jacobmatthe.ws!\n");
    println!("Listening on {}:{}\n\n-------", config.host, config.port);

    return server.await;
}
