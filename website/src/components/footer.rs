use chrono::Datelike;

pub fn render() -> String {
    return format!(
        r#"
    <footer class="flex flex-row justify-center items-center text-sm sticky bottom-0 bg-zinc-900">
        <div class="flex-1 flex flex-row items-center justify-center py-5 max-w-5xl text-sky-400 border-t border-sky-600">
          <p>Copyright © Jacob Ian Matthews {year}</p>
       </div>
    </footer>
    "#,
        year = chrono::Utc::now().year()
    );
}
