pub fn new() -> String {
    return format!(
        r#"
    <header class="flex flex-row justify-center sticky top-0 bg-zinc-900 text-sky-400 pt-10">
        <div class="flex-1 border-b border-sky-600 relative max-w-5xl">
            <div class="w-full flex flex-row items-center max-w-5xl px-5 absolute -bottom-3">
                <a href="/" alt="Home" class="text-md bg-zinc-900 px-1">jacobmatthe.ws</a>
                <div class="flex-1"></div>
                <nav class="flex flex-row justify-start items-center gap-5">
                    {home}
                    {blog}
                    {about}
                </nav>
                <div class="flex-1"></div>
                <div class="flex flex-row justify-end">
                    <button class="bg-zinc-900 px-1" alt="Search">Search</button>
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
