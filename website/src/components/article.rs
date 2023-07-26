use chrono::{DateTime, Datelike, Utc};

pub struct ArticleBuilder {
    title: String,
    content: String,
    published_at: Option<DateTime<Utc>>,
}

impl ArticleBuilder {
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = title;
        return self;
    }

    pub fn content(&mut self, content: String) -> &mut Self {
        self.content = content;
        return self;
    }

    pub fn published_at(&mut self, published_at: Option<DateTime<Utc>>) -> &mut Self {
        self.published_at = published_at;
        return self;
    }

    fn render_pubdate(&self) -> String {
        return if let Some(datetime) = self.published_at {
            format!(
                r#"
            <time class="text-slate-400" datetime="{iso}" pubdate>{plaintext}</time>
                "#,
                iso = datetime.to_rfc3339(),
                plaintext = format!("{}", datetime.format("%d %B, %Y"))
            )
        } else {
            String::new()
        };
    }

    pub fn render(&self) -> String {
        return format!(
            r#"
<article class="flex-1 flex min-w-full flex-col prose prose-invert 
    prose-headings:text-sky-100
    prose-p:text-sky-100
    prose-pre:bg-transparent prose-pre:p-0 prose-pre:rounded-none
    prose-code:p-5 prose-code:bg-zinc-800 prose-code:border prose-code:border-sky-600 prose-code:before:content-[] prose-code:after:content-[]">
    <h1 class="font-extrabold lowercase text-base before:content-['$'] before:mr-2">{title}</h1>
    {pubdate}
    {content}
</article>
            "#,
            title = self.title,
            pubdate = self.render_pubdate(),
            content = self.content
        );
    }
}

pub fn builder() -> ArticleBuilder {
    return ArticleBuilder {
        title: String::new(),
        content: String::new(),
        published_at: None,
    };
}
