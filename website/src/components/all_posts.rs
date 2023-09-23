use chrono::Datelike;

use crate::{
    cms::{posts::Post, Client},
    errors::Error,
};

pub async fn render(client: &Client) -> Result<String, Error> {
    let posts = get_all_posts(client).await?;
    let list = render_year_month_list(posts)?;

    return Ok(format!(
        r#"
        <h2 class="my-5 text-2xl font-bold">All Posts:</h2>
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

fn render_year_month_list(mut posts: Vec<Post>) -> Result<String, Error> {
    if posts.len() == 0 {
        return Ok(String::from(
            "<p>There's nothing here just yet, but stay tuned!</p>",
        ));
    }
    posts.sort_by(|a, b| b.published_at.cmp(&a.published_at));

    let mut groups: Vec<Vec<Vec<&Post>>> = Vec::new();
    let mut i = 0;
    while i < posts.len() {
        let cur = &posts[i];
        if i == 0 {
            groups.push(vec![vec![cur]]);
            i += 1;
            continue;
        }
        let prev = &posts[i - 1];
        if prev.published_at.year() != cur.published_at.year() {
            groups.push(vec![vec![cur]]);
            i += 1;
            continue;
        }

        let year = &groups.len() - 1;
        if prev.published_at.month() != cur.published_at.month() {
            groups[year].push(vec![cur]);
            i += 1;
            continue;
        }

        let months = groups
            .get_mut(year)
            .ok_or(Error::Internal(String::from("Could not get all posts")))?;
        let month = months.len() - 1;
        months[month].push(cur);
        i += 1;
    }

    return Ok(groups.into_iter().map(|y| {
        return format!(
            r#"
            <details open>
                <summary class="cursor-pointer">{year}</summary>
                <div class="flex flex-col px-5 py-2">
                    {months}
                </div>
            </details>
            "#,
            year = y[0][0].published_at.year(),
            months = y.into_iter().map(|m| {
                return format!(
                            r#"
                            <details open class="mb-1">
                                <summary class="cursor-pointer">{month}</summary>
                                <ul class="flex flex-col px-5 py-2 gap-2 list-[square]">
                                    {p}
                                </ul>
                            </details>
                            "#,
                    month = m[0].published_at.format("%B"),
                    p = m.into_iter().map(|post| {
                                return format!(
                                    r#"
                                    <li><a class="flex-1 text-sky-300 transition-colors hover:text-sky-100" href="/{stub}">{name}</a></li>
                                    "#,
                                    stub = post.stub,
                                    name = post.title
                                );
                    }).collect::<Vec<String>>().join("\n")
                );
            }).collect::<Vec<String>>().join("\n")
        )
    }).collect::<Vec<String>>().join("\n"));
}
