use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use backend::config::Config;
use backend::db;
use backend::handlers;
use std::{env, process};

async fn hello_world() -> impl Responder {
    return HttpResponse::Ok().body("Hello world");
}

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
            .route("/", web::get().to(hello_world))
            .service(web::scope("/auth").configure(handlers::auth::config))
            .service(web::scope("/posts").configure(handlers::posts::config))
    })
    .bind((config.host.clone(), config.port.clone()))?
    .run();

    println!("Listening on {}:{}", &config.host, &config.port);

    return server.await;
}
