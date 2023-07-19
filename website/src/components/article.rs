pub fn new(content: String) -> String {
    return format!(
        r#"
    <article class="flex-1 flex flex-col prose prose-invert 
        prose-headings:text-sky-100
        prose-p:text-sky-100
        prose-h1:font-extrabold prose-h1:before:content-['$'] prose-h1:before:mr-2 prose-h1:lowercase prose-h1:text-base
        prose-pre:bg-transparent prose-pre:p-0 prose-pre:rounded-none
        prose-code:p-5 prose-code:bg-zinc-800 prose-code:border prose-code:border-sky-600 prose-code:before:content-[] prose-code:after:content-[]">
        {content}
    </article>
    "#
    );
}
