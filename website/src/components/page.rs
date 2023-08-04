use crate::html::HtmlResponse;

use super::{footer, header};

/// Returns a completed HTML page template from an HtmlResponse
pub fn from_response(res: &HtmlResponse) -> String {
    return format!(
        r#"
        <!DOCTYPE html>
        <html>
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
                <script src="https://cdn.tailwindcss.com?plugins=typography"></script> <!-- Remove in prod -->
                <script>
                    tailwind.config = {{
                      theme: {{
                        fontFamily: {{
                          mono: ['JetBrains Mono', 'mono']
                        }},
                      }},
                    }}
                </script>
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/tokyo-night-dark.min.css">
                <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js"></script>
                <script>hljs.highlightAll();</script>
                <script src="https://unpkg.com/hyperscript.org@0.9.9"></script>
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
