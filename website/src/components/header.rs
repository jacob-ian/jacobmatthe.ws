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
        home = nav_item(NavItem {
            href: "/",
            label: "Home",
            id: "nav-home",
            highlight_condition: "'/' is"
        }),
        blog = nav_item(NavItem {
            href: "/blog",
            label: "Blog",
            id: "nav-blog",
            highlight_condition: "['/', '/about'] does not contain"
        }),
        about = nav_item(NavItem {
            href: "/about",
            label: "About",
            id: "nav-about",
            highlight_condition: "'/about' is"
        })
    );
}

struct NavItem {
    href: &'static str,
    label: &'static str,
    id: &'static str,
    highlight_condition: &'static str,
}

fn nav_item(item: NavItem) -> String {
    return format!(
        r#"<a _="init 
            if {condition} the pathname of the location of the window 
                add .text-sky-100 to me
            else 
                remove .text-sky-100 
            end" id="{id}" href="{href}" alt="{label}" class="bg-zinc-900 px-1 text-md hover:text-sky-100">{label}</a>"#,
        id = item.id,
        href = item.href,
        label = item.label,
        condition = item.highlight_condition
    );
}
