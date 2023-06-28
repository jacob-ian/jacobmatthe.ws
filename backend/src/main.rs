use actix_session;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::time::Duration;
use actix_web::cookie::{Key, SameSite};
use actix_web::{error, web, App, HttpServer, ResponseError};
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

    let pool = match db::setup_pool_from_config(&config).await {
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
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(app_config.auth.session_key.as_bytes()),
                )
                .cookie_name(String::from("sid"))
                .cookie_same_site(SameSite::Strict)
                .cookie_http_only(true)
                .cookie_secure(app_config.auth.secure_cookies)
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(7)))
                .build(),
            )
            .route("/", web::get().to(handlers::health_check))
            .service(web::scope("/auth").configure(handlers::auth::config))
            .service(web::scope("/users").configure(handlers::users::config))
            .service(web::scope("/posts").configure(handlers::posts::config))
            .service(web::scope("/drafts").configure(handlers::drafts::config))
            .service(web::scope("/uploads").configure(handlers::uploads::config))
    })
    .bind((config.host.clone(), config.port.clone()))?
    .run();

    println!("-------\nSuccessfully started jacobmatthe.ws/api!\n");
    println!(
        "Server is listening on {}:{}\n-------",
        &config.host, &config.port
    );

    return server.await;
}
