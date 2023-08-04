use crate::{components::article, errors::Error, html::HtmlResponse};

// TODO: This needs to be a Page type in the CMS
pub async fn about() -> Result<HtmlResponse, Error> {
    return Ok(HtmlResponse::builder()
        .title(String::from("About | Jacob Matthews"))
        .body(
            article::builder()
                .title(String::from("About"))
                .content(String::from(
                    r#"
                <p>This is something about me</p>
                    "#,
                ))
                .render(),
        )
        .build());
}
