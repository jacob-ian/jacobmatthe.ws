pub fn new() -> String {
    struct Item {
        href: &'static str,
        label: &'static str,
    }
    return vec![
        Item {
            href: "/",
            label: "Home",
        },
        Item {
            href: "/page",
            label: "Page",
        },
    ]
    .iter()
    .map(|l| {
        format!(
            r#"<a href="{href}" alt="{label}">{label}</a>"#,
            label = l.label,
            href = l.href
        )
    })
    .collect::<Vec<String>>()
    .join("")
    .to_string();
}
