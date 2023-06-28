use actix_web::{error, web, App, HttpServer, ResponseError};
use backend::auth::sessions;
use backend::config::Config;
use backend::db;
use backend::errors::Error;
use backend::handlers;
use std::process;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting up...");
    let config = match Config::from_toml() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let pool = match db::pool_from_config(&config).await {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let app_config = web::Data::new(config.clone());
    let json_config = web::JsonConfig::default().error_handler(|err, _| {
        let mapped = Error::from_json_payload(err);
        error::InternalError::from_response(mapped.clone(), mapped.error_response()).into()
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::clone(&app_config))
            .app_data(json_config.clone())
            .wrap(sessions::middleware_from_config(&app_config))
            .configure(handlers::config)
    })
    .bind((config.host.clone(), config.port.clone()))?
    .run();

    println!("\n-------\n\nSuccessfully started jacobmatthe.ws/api!\n");
    println!("Listening on {}:{}\n\n-------", &config.host, &config.port);

    return server.await;
}
