pub fn new(code: &'static str) -> String {
    return format!(
        r#"
        <code class="font-mono my-2 p-5 bg-zinc-800 border border-sky-600">{code}</code>
    "#,
        code = code
    );
}
