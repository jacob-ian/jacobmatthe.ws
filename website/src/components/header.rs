pub fn render() -> String {
    return format!(
        r#"
        <header class="flex flex-row justify-center bg-zinc-900 text-sky-400 pt-5 sm:pt-10 sm:sticky sm:top-0">
            <div class="flex-1 border-b border-sky-600 relative max-w-5xl">
                <div class="px-5 flex flex-col items-center sm:flex-row sm:w-full sm:absolute sm:-bottom-3">
                    <div class="flex-1 flex flex-row justify-start">
                        <a href="/" alt="Home" class="text-md bg-zinc-900 px-1">jacobmatthe.ws</a>
                    </div>
                    <nav _="init
                        if ['/', '/about'] does not contain the pathname of the location of the window 
                            add .text-sky-100 to #nav-blog
                        else 
                            remove .text-sky-100 from #nav-blog
                        end
                    "
                    class="flex-1 flex flex-row justify-center items-center gap-5">
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
        home = nav_item(NavItem {
            href: "/",
            label: "Home",
            id: "nav-home",
            highlight_on_path: true
        }),
        blog = nav_item(NavItem {
            href: "/blog",
            label: "Blog",
            id: "nav-blog",
            highlight_on_path: false
        }),
        about = nav_item(NavItem {
            href: "/about",
            label: "About",
            id: "nav-about",
            highlight_on_path: true
        })
    );
}

struct NavItem {
    href: &'static str,
    label: &'static str,
    id: &'static str,
    highlight_on_path: bool,
}

fn nav_item(item: NavItem) -> String {
    let mut highlight_script = String::new();

    if item.highlight_on_path {
        highlight_script = format!(
            r#"_="init 
            if the pathname of the location of the window is '{href}' 
                add .text-sky-100 to me
            else 
                remove .text-sky-100 
            end""#,
            href = &item.href
        );
    }

    return format!(
        r#"<a {highlight} id="{id}" href="{href}" alt="{label}" class="bg-zinc-900 px-1 text-md hover:text-sky-100">{label}</a>"#,
        id = item.id,
        href = item.href,
        label = item.label,
        highlight = highlight_script
    );
}
