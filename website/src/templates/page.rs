use super::{footer, header};

pub fn new(title: &'static str, description: &'static str, body: &'static str) -> String {
    let header = header::new();
    let footer = footer::new();
    return format!(
        r#"
        <!DOCTYPE html>
        <html class="subpixel-antialiased">
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
            <style type="text/css">
                html, body {{
                    font-family: "Noto Serif", serif;
                }}
                code {{
                    font-family: "JetBrains Mono", monospace;
                }}
            </style>
            <body class="bg-zinc-900 text-neutral-50">
                {header}
                {body}
                {footer}
            </body>
        </html>
        "#,
        title = title,
        description = description,
        header = header,
        body = body,
        footer = footer
    );
}
