use crate::{components::article, errors::Error, html::HtmlResponse};

// TODO: Move to page content type in CMS
pub async fn now() -> Result<HtmlResponse, Error> {
    return Ok(HtmlResponse::builder()
        .title(String::from("Now | Jacob Matthews"))
        .body(
            article::builder()
                .title(String::from("Now"))
                .content(String::from(
                    r#"
            <p>This is something about me</p>
                "#,
                ))
                .render(),
        )
        .build());
}
