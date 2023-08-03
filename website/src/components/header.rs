pub fn render() -> String {
    return format!(
        r#"
        <header class="flex flex-row justify-center bg-zinc-900 text-sky-400 pt-5 sm:pt-10 sm:sticky sm:top-0">
            <div class="flex-1 border-b border-sky-600 relative max-w-5xl">
                <div class="px-5 flex flex-col items-center sm:flex-row sm:w-full sm:absolute sm:-bottom-3">
                    <div class="flex-1 flex flex-row justify-start">
                        <a href="/" alt="Home" class="text-md bg-zinc-900 px-1">jacobmatthe.ws</a>
                    </div>
                    <nav class="flex-1 flex flex-row justify-center items-center gap-5">
                        {home}
                        {blog}
                        {about}
                    </nav>
                    <div class="flex-1 flex flex-row justify-end">
                        <!--<button class="bg-zinc-900 px-1" alt="Search">Search</button>-->
                    </div>
                </div>
            </div>
        </header>
        "#,
        home = nav_item("/", "Home"),
        blog = nav_item("/blog", "Blog"),
        about = nav_item("/about", "About")
    );
}

fn nav_item(href: &'static str, label: &'static str) -> String {
    return format!(
        r#"<a href="{href}", alt="{label}" class="bg-zinc-900 px-1 text-md transition-colors hover:text-sky-100">{label}</a>"#,
        href = href,
        label = label
    );
}
