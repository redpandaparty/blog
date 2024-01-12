fn main() -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(std::env::current_dir()?)? {
        let entry = entry?;
        if entry.file_type()?.is_file()
            && entry.path().extension() == Some(std::ffi::OsStr::new("md"))
        {
            let output_path = entry.path().with_extension("html");
            std::fs::write(
                &output_path,
                format!(
                    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta name="apple-mobile-web-app-capable" content="yes">
    <title>{}</title>
    <style>
        html, body {{
            overflow-x: hidden;
        }}
        @media (prefers-color-scheme: dark) {{
            body {{
                color: #fafafa;
                background-color: #000;
            }}
            :link, :visited, :visited:active {{
                color: #2f81f7;
            }}
        }}
        body {{
            position: relative;
            box-sizing: border-box;
            -webkit-box-sizing: border-box;
            -moz-box-sizing: border-box;
        }}
        pre {{
            overflow-x: scroll;
        }}
        img {{
            max-width: 100%;
        }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
                    entry
                        .path()
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    markdown::to_html_with_options(
                        &std::fs::read_to_string(entry.path())?,
                        &markdown::Options::gfm(),
                    )?,
                ),
            )?;
            println!("{} -> {}", entry.path().display(), output_path.display());
        }
    }
    Ok(())
}
