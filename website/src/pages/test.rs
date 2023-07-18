use actix_web::Responder;

use crate::html::HtmlResponse;

pub async fn page() -> impl Responder {
    return HtmlResponse::builder()
        .title("How to page a page")
        .body("<p>Hi there</p>".to_string())
        .build();
}
