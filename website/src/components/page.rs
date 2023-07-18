use crate::html::HtmlResponse;

use super::{footer, header};

/// Returns a completed HTML page template from an HtmlResponse
pub fn from_response(res: &HtmlResponse) -> String {
    let header = header::new();
    let footer = footer::new();
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
                <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono&family=Noto+Serif:ital,wght@0,400;0,600;1,400&display=swap" rel="stylesheet">

                <!-- Stylesheets -->
                <script src="https://cdn.tailwindcss.com"></script> <!-- Remove in prod -->
                <script>
                    tailwind.config = {{
                      theme: {{
                         fontFamily: {{
                          serif: ['Noto Serif', 'serif'],
                          mono: ['JetBrains Mono', 'mono']
                        }},
                      }}
                    }}
                </script>
            </head>
            <body class="min-h-screen flex flex-col bg-zinc-900 font-mono text-sky-100">
                {header}
                <main class="flex flex-row flex-1 justify-center">
                    <div class="p-10 flex-1 max-w-5xl border-x border-sky-600">
                        {body}
                    </div>
                </main>
                {footer}
            </body>
        </html>
        "#,
        title = res.head.title,
        description = res.head.description,
        header = header,
        body = res.body,
        footer = footer
    );
}
