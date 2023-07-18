use chrono::Datelike;

pub fn new() -> String {
    return format!(
        r#"
    <footer class="flex flex-row justify-center items-center p-5 text-sm font-bold border-t border-sky-600 sticky bottom-0">
        <div class="flex-1 flex flex-row items-center justify-center gap-0.5 max-w-5xl text-sky-400">
          <p>Copyright © Jacob Ian Matthews {year}</p>
       </div>
    </footer>
    "#,
        year = chrono::Utc::now().year()
    );
}
