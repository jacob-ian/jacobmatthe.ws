pub fn new() -> String {
    return format!(
        r#"
    <header class="flex flex-row justify-center items-center sticky top-0 bg-zinc-900 border-b border-sky-600 text-sky-400">
        <div class="flex-1 flex flex-row items-center justify-start gap-0.5 max-w-5xl">
            <a class="flex-0 text-md font-extrabold px-5">jacobmatthe.ws</a>
            <div class="flex-1"></div>
            <nav class="flex-0 flex flex-row justify-start items-center gap-1.5 divide-x divide-sky-600">
                {blog}
                {about}
            </nav>
        </div>
    </header>
    "#,
        blog = nav_item("/", "Blog"),
        about = nav_item("/about", "About")
    );
}

fn nav_item(href: &'static str, label: &'static str) -> String {
    return format!(
        r#"<a href="{href}", alt="{label}" class="py-2 px-5 my-5 font-bold text-md transition-colors hover:text-sky-200">{label}</a>"#,
        href = href,
        label = label
    );
}
