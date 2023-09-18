use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    cms::{posts::Post, Client},
    errors::Error,
};

pub async fn render(client: &Client) -> Result<String, Error> {
    let posts = get_latest_posts(&client).await?;

    let mut list = String::from("<p><em>There's nothing here just yet, but stay tuned!</em></p>");

    if posts.len() > 0 {
        list = posts
        .into_iter()
        .map(|p| {
            format!(
                r#"
                <a class="text-sky-300 before:content-['>'] before:mr-2 flex flex-row mb-1 transition-colors hover:text-sky-100" href="/{stub}">
                    <div>{name}</div>
                    <div class="flex-1"></div>
                    <div>{date}</div>
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
    }

    return Ok(format!(
        r#"
        <h2 id="latest" class="my-5 text-2xl font-bold">Latest Posts:</h2>
        {posts}
        "#,
        posts = list
    ));
}

#[derive(Serialize)]
struct LatestPostsParams {
    limit: i64,
}

async fn get_latest_posts(client: &Client) -> Result<Vec<Post>, Error> {
    return client
        .get()
        .path(String::from("posts"))
        .query(LatestPostsParams { limit: 10 })?
        .json::<Vec<Post>>()
        .await;
}

fn format_time_ago(date: DateTime<Utc>) -> String {
    let duration = Utc::now().signed_duration_since(date);

    if duration.num_weeks() > 1 {
        return format!("{} weeks ago", duration.num_weeks());
    }

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
