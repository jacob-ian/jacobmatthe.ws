use std::collections::HashMap;

use chrono::Datelike;

use crate::{
    cms::{posts::Post, Client},
    errors::Error,
};

pub async fn render(client: &Client) -> Result<String, Error> {
    let posts = get_all_posts(client).await?;
    let list = group_by_year_month(posts)
        .into_iter()
        .map(|(y, m)| {
            return format!(
                r#"
                <details open>
                    <summary class="cursor-pointer">{year}</summary>
                    <div class="flex flex-col px-5 py-2">
                        {months}
                    </div>
                </details>
                "#,
                year = y,
                months = m
                    .into_iter()
                    .map(|(m, p)| {
                        return format!(
                            r#"
                            <details open class="mb-1">
                                <summary class="cursor-pointer">{month}</summary>
                                <ul class="flex flex-col px-5 py-2 gap-2 list-[square]">
                                    {posts}
                                </ul>
                            </details>
                            "#,
                            month = m,
                            posts = p
                                .into_iter()
                                .map(|post| {
                                    return format!(
                                        r#"
                                        <li><a class="flex-1 text-sky-300 transition-colors hover:text-sky-100" href="/{href}">{name}</a></li>
                                        "#,
                                        href = post.stub,
                                        name = post.title
                                    );
                                })
                                .collect::<Vec<String>>()
                                .join("\n")
                        );
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        })
        .collect::<Vec<String>>()
        .join("\n");

    return Ok(format!(
        r#"
        <h2 class="my-5 text-lg">All Posts:</h2>
        {list}
        "#,
        list = list
    ));
}

// TODO: This will become really inefficient - request less fields
async fn get_all_posts(client: &Client) -> Result<Vec<Post>, Error> {
    return client
        .get()
        .path(String::from("posts"))
        .json::<Vec<Post>>()
        .await;
}

fn group_by_year_month(posts: Vec<Post>) -> HashMap<i32, HashMap<String, Vec<Post>>> {
    let mut map: HashMap<i32, HashMap<String, Vec<Post>>> = HashMap::new();
    for post in posts {
        let year = post.published_at.year();
        let month = post.published_at.format("%B").to_string();

        let year_group = map.entry(year).or_insert(HashMap::new());
        let month_group = year_group.entry(month).or_insert(Vec::new());
        month_group.push(post);
    }
    return map;
}
