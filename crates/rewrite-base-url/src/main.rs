use std::{
    error::Error,
    fs::{read_to_string, write},
    path::PathBuf,
};

use clap::Parser;
use lol_html::{element, rewrite_str, RewriteStrSettings};
use url::Url;
use walkdir::WalkDir;

fn url_with_trailing_slash(url: &str) -> String {
    if url.ends_with('/') {
        url.to_string()
    } else {
        format!("{}/", url)
    }
}

/// Rewrites the base URL of all `href="..."` and `src="..."` attributes
/// across all files in the input directory. Useful for remapping static
/// sites to a different base URL.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    html_dir: PathBuf,
    #[clap(short, long)]
    base_url: String,
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let base_url = Url::parse("http://localhost/")?.join(&url_with_trailing_slash(&args.base_url))?;

    for entry in WalkDir::new(&args.html_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if path.extension().map(|e| e == "html").unwrap_or(false) {
            if args.verbose {
                eprintln!("Processing file: {:?}", path);
            }

            let html = read_to_string(path)?;

            let element_content_handlers = vec![
                element!("[href]", |el| {
                    let href = el.get_attribute("href").unwrap();
                    if href.starts_with("/") {
                        let href_url = base_url.join(href.strip_prefix("/").unwrap())?;
                        let href_rest = href_url.as_str().strip_prefix("http://localhost").unwrap();
                        el.set_attribute("href", href_rest)?;

                        if args.verbose {
                            eprintln!("Rewrote href: {} -> {}", href, href_rest);
                        }
                    }

                    Ok(())
                }),
                element!("[src]", |el| {
                    let src = el.get_attribute("src").unwrap();
                    if src.starts_with("/") {
                        let src_url = base_url.join(src.strip_prefix("/").unwrap())?;
                        let src_rest = src_url.as_str().strip_prefix("http://localhost").unwrap();
                        el.set_attribute("src", src_rest)?;

                        if args.verbose {
                            eprintln!("Rewrote src: {} -> {}", src, src_rest);
                        }
                    }

                    Ok(())
                }),
            ];
            let new_html = rewrite_str(
                &html,
                RewriteStrSettings {
                    element_content_handlers,
                    ..RewriteStrSettings::default()
                },
            )?;

            eprintln!("Writing file: {:?}", path);
            write(path, new_html)?;
        } else {
            if args.verbose {
                eprintln!("Skipping non-HTML file: {:?}", path);
            }
        }
    }

    eprintln!("All done!");
    Ok(())
}
