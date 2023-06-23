use actix_web::{web, App, HttpServer};
use backend::config::Config;
use backend::db;
use backend::handlers;
use std::{env, process};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match Config::from_env(env::vars()) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let pool = match db::setup_pool_from_config(&config).await {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(handlers::health_check))
            .service(web::scope("/auth").configure(handlers::auth::config))
            .service(web::scope("/posts").configure(handlers::posts::config))
            .service(web::scope("/drafts").configure(handlers::drafts::config))
    })
    .bind((config.host.clone(), config.port.clone()))?
    .run();

    println!("-------\nSuccessfully started jacobmatthe.ws/api!\n-------");
    println!("Server is listening on {}:{}", &config.host, &config.port);

    return server.await;
}
