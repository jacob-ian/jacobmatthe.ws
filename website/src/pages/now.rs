use chrono::Utc;

use crate::{components::article, errors::Error, html::HtmlResponse};

// TODO: Move to page content type in CMS
pub async fn now() -> Result<HtmlResponse, Error> {
    return Ok(HtmlResponse::builder()
        .title(String::from("Now | Jacob Matthews"))
        .body(
            article::builder()
                .title(String::from("Now"))
                .published_at(Some(
                    "2023-09-18T00:00:00Z"
                        .parse::<chrono::DateTime<Utc>>()
                        .map_err(|_| Error::Internal(String::from("Could not parse date")))?,
                ))
                .content(String::from(
                    r#"
                    <p><a href="https://nownownow.com/about" rel="nofollow" target="_blank">What is this?</a></p>
                    <h2 class="mt-5">Where am I?</h2>
                    <p>I'm living in Sydney, currently moving into a new apartment thanks to a substantial increase in rent. The Sydney rental market has certainly changed since I moved here in 2021, but I don't want to leave it just yet, so I'll pay to play.</p>
                    <h2>What am I working on?</h2>
                    <p>I'm working at The Mintable, creating new technologies to help managers be more effective. At the moment this includes a Slack app that <a target="_blank" ref="nofollow" href="https://beta.getmintable.com/platform/manager-assistant">you can install here.</a><p>
                    <p>I have finished rewriting my website with as close to zero JavaScript as possible (blog post coming soon) and am writing blog posts here, so stay tuned!</p>
                    <p>I also have another project simmering in the background that I will start building before the end of the month. A teaser: how can we feel better understood?</p>
                    <h2>What am I reading?</h2>
                    <p>I'm currently reading <em>Think and Grow Rich</em> by Napoleon Hill, which is helping me get back to the basics of goal-setting and strategising.</p>
                    "#,
                ))
                .render(),
        )
        .build());
}
