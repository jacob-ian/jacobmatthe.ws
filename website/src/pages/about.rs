use actix_web::Responder;

use crate::{components::article, html::HtmlResponse};

pub async fn about() -> impl Responder {
    return HtmlResponse::builder()
        .title(String::from("About | Jacob Matthews"))
        .body(
            article::builder()
                .title(String::from("About"))
                .content(format!(
                    r#"
<p>blah</p>
<img alt="test" src="/uploads/test.jpg" />
                    "#,
                ))
                .render(),
        )
        .build();
}
