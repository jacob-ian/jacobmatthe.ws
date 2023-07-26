use actix_web::Responder;

use crate::{components::article, html::HtmlResponse};

pub async fn blog() -> impl Responder {
    return HtmlResponse::builder()
        .title(String::from("Blog | Jacob Matthews"))
        .body(
            article::builder()
                .title(String::from("Blog"))
                .content(
                    r#"
<p>Posts go here</p>
                    "#
                    .to_string(),
                )
                .render(),
        )
        .build();
}
