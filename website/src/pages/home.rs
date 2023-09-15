use actix_web::web;

use crate::{
    cms::Client,
    components::{article, latest_posts},
    errors::Error,
    html::HtmlResponse,
};

pub async fn home(client: web::Data<Client>) -> Result<HtmlResponse, Error> {
    return Ok(HtmlResponse::builder()
        .title(String::from("Home | Jacob Matthews"))
        .body(format!(
            r#"
            {content}
            {latest}
            <div class="flex-1"><div>
            <div class="min-w-full prose prose-invert prose-p:text-sky-100">
                <hr class="my-10"></hr>
                <p class="text-center">If you want to say hi, email me at <a href="mailto:jacob@jacobmatthe.ws">jacob@jacobmatthe.ws</a>. I don't bite!<p>
            </div>
            "#,
            content = article::builder()
                .title(String::from("home"))
                .content(
                    r#"
                <h2 class="text-center mt-5 mb-10">Welcome to my website!</h2>
                <p>My name is Jacob and I like to make things, including software. On this website you can read what I think about software, business, and some of the more important things.</p> 
                <div class="mt-10"></div>
                <p>To learn a little more about me, <a href="/about">see my <em>about</em> page.</a></p>
                <div class="mt-10"></div>
                <p>If you'd like to see what I'm currently working on, you can <a href="/now">visit my <em>now</em> page.</a></p>
                <div class="mt-10"></div>
                <p>To read my musings, check out my <em>latest posts</em> below, or <a href="/blog">visit my <em>blog</em>.</a></p>
                <hr class="mt-10 mb-5"></hr>
                    "#
                    .to_string()
                )
                .render(),
            latest = latest_posts::render(&client).await?,
        ))
        .build());
}
