pub fn render(title: &str) -> String {
    return format!(
        r#"<h1 class="font-extrabold lowercase text-base before:content-['$'] before:mr-2">{text}</h1>"#,
        text = title
    );
}
