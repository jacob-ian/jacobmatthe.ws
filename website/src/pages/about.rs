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
                    <h2>A Short Bio</h2>
                    <p>I grew up in rural Perth, Western Australia, and moved to Sydney in 2021. I first started programming when I was 10 years old, writing batch scripts on the Primary School computers when I should have been doing assignments.</p>
                    "#,
                ))
                .render(),
        )
        .build());
}
