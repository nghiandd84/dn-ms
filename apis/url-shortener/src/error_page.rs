/// Renders an HTML error page for expired or inactive links.
pub fn render_error_page(message: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Link Unavailable</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            background-color: #f5f5f5;
            color: #333;
        }}
        .container {{
            text-align: center;
            padding: 2rem;
            max-width: 500px;
        }}
        h1 {{
            font-size: 2rem;
            margin-bottom: 1rem;
            color: #e74c3c;
        }}
        p {{
            font-size: 1.1rem;
            line-height: 1.6;
            color: #666;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Link Unavailable</h1>
        <p>{}</p>
    </div>
</body>
</html>"#,
        message
    )
}
