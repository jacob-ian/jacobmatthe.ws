use crate::html::HtmlResponse;

use super::{footer, header};

/// Returns a completed HTML page template from an HtmlResponse
pub fn from_response(res: &HtmlResponse) -> String {
    return format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{title}</title>
                <meta name="description" content="{description}" />

                <!-- Fonts -->
                <link rel="preconnect" href="https://fonts.googleapis.com">
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,400;0,800;1,400&display=swap" rel="stylesheet">

                <!-- Stylesheets -->
                <link rel="stylesheet" href="/static/highlightjs@11.7.0-tokyo-night-dark.min.css">
                <link rel="stylesheet" href="/static/tailwind.min.css">

                <!-- Scripts -->
                <script src="/static/highlightjs@11.7.0.min.js"></script>
                <script>hljs.highlightAll();</script>
                <script src="/static/hyperscript@0.9.9.min.js"></script>
            </head>
            <body class="min-h-screen flex flex-col bg-zinc-900 font-mono text-sky-100">
                {header}
                <main class="flex flex-row flex-1 justify-center">
                    <div class="p-10 flex-1 flex flex-col max-w-5xl border-x border-sky-600">
                        {body}
                    </div>
                </main>
                {footer}
            </body>
        </html>
        "#,
        title = res.head.title,
        description = res.head.description,
        header = header::render(),
        body = res.body,
        footer = footer::render()
    );
}
