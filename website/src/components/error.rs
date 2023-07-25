pub fn render(title: &str, content: String) -> String {
    return format!(
        r#"
        <div class="flex-1 flex flex-col">
            <h1 class="font-extrabold before:content-['$'] before:mr-2 lowercase text-base">{title}</h1>
            <div class="flex-1 flex flex-col justify-center items-center">
            <pre>
         __
 _(\    |@@|
(__/\__ \--/ __
   \___|----|  |   __
       \ }}{{ /\ )_ / _\
       /\__/\ \__O (__
      (--/\--)    \__/
      _)(  )(_
     `---''---`
            </pre>
            {content}
            </div>
            <div class="flex-1"></div>
        </div>
        "#,
        title = title,
        content = content
    );
}
