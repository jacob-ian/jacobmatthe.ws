use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{time::Duration, Key, SameSite};

use crate::config::Config;

pub fn middleware_from_config(config: &Config) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(),
        Key::from(config.auth.session_key.as_bytes()),
    )
    .cookie_name(String::from("sid"))
    .cookie_same_site(SameSite::Strict)
    .cookie_http_only(true)
    .cookie_secure(config.auth.secure_cookies)
    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(7)))
    .build()
}
