use actix_web::web;
use chrono::{DateTime, Utc};

use crate::{
    cms::{posts::Post, Client},
    components::title,
    errors::Error,
    html::HtmlResponse,
};

async fn get_posts(client: &Client) -> Result<Vec<Post>, Error> {
    return client
        .get()
        .path(String::from("posts"))
        .json::<Vec<Post>>()
        .await;
}

pub async fn blog(client: web::Data<Client>) -> Result<HtmlResponse, Error> {
    let posts = get_posts(&client)
        .await?
        .into_iter()
        .map(|p| {
            format!(
                r#"
                <a class="text-sky-300 before:content-['>'] before:mr-2 flex flex-row" href="/{stub}">
                    <div>{name}</div>
                    <div class="flex-1"></div>
                    <div class="text-sky-300">{date}</div>
                </a>
                "#,
                stub = p.stub,
                name = p.title,
                date = format_time_ago(p.published_at)
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
        .to_string();

    return Ok(HtmlResponse::builder()
        .title(String::from("Blog | Jacob Matthews"))
        .body(format!(
            r#"
            {title}
            <h2 class="my-5">Latest Posts:</h2>
            {posts}
            "#,
            title = title::render("Blog"),
            posts = posts
        ))
        .build());
}

fn format_time_ago(date: DateTime<Utc>) -> String {
    let duration = Utc::now().signed_duration_since(date);

    if duration.num_days() > 1 {
        return format!("{} days ago", duration.num_days());
    }

    if duration.num_days() == 1 {
        return String::from("Yesterday");
    }

    if duration.num_hours() >= 1 {
        let mut plural = String::new();
        if duration.num_hours() > 1 {
            plural = String::from("s");
        }
        return format!("{} hour{} ago", duration.num_hours(), plural);
    }

    return String::from("Under an hour ago");
}
