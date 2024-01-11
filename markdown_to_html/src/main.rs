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
                    r#"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <title>{}</title>
</head>
<body>
{}
</body>
</html>"""#,
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
