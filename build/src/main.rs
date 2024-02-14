use askama::Template;
use clap::Parser;
use std::{
    error::Error,
    fs::{create_dir_all, remove_dir_all, rename, write},
    panic::catch_unwind,
    path::{Path, PathBuf},
};
use typst::{layout::Frame, visualize::Color};
use typst_docs::{provide, Html, PageModel, Resolver};
use typst_render::render;
use walkdir::WalkDir;

struct MyResolver<'a> {
    out_dir: &'a Path,
}
impl<'a> Resolver for MyResolver<'a> {
    fn commits(&self, from: &str, to: &str) -> Vec<typst_docs::Commit> {
        eprintln!("commits({from}, {to})");
        vec![]
    }
    fn example(
        &self,
        hash: u128,
        source: Option<typst_docs::Html>,
        document: &[Frame],
    ) -> typst_docs::Html {
        eprintln!(
            "example(0x{hash:x}, {:?} chars, a document)",
            source.as_ref().map(|s| s.as_str().len())
        );

        Html::new("".to_string())
    }
    fn image(&self, filename: &str, data: &[u8]) -> String {
        eprintln!("image({filename}, {} bytes)", data.len());
        format!("/assets/docs/{filename}")
    }
    fn link(&self, link: &str) -> Option<String> {
        eprintln!("link({link})");
        None
    }
}

#[derive(Template)]
#[template(path = "html.html")]
struct HtmlTemplate<'a> {
    page: &'a PageModel,
    pages: &'a [&'a PageModel],
}

#[derive(Template)]
#[template(path = "func.html")]
struct FuncTemplate;

fn pages_flat<'a>(pages: &'a [&'a PageModel]) -> Vec<&'a PageModel> {
    fn pages_flat_helper<'a>(page: &'a PageModel, pages: &mut Vec<&'a PageModel>) {
        pages.push(page);
        pages.extend(page.children.iter());
    }
    let mut all_pages: Vec<&'a PageModel> = Vec::new();
    for page in pages {
        pages_flat_helper(page, &mut all_pages);
    }
    all_pages
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Where to put the final site files
    #[arg(short, long, default_value = "_site")]
    out_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let own_pages = provide(&MyResolver {
        out_dir: args.out_dir.as_path(),
    });
    let pages: Vec<_> = own_pages.iter().collect();
    let all_pages = pages_flat(pages.as_slice());

    for page in all_pages {
        let path = args.out_dir.clone();
        let mut route = page.route.to_string();
        if route.ends_with("/") {
            route.push_str("index.html");
        }
        if route.starts_with("/") {
            let _ = route.remove(0);
        }
        let path = path.join(route);

        let html = HtmlTemplate {
            page,
            pages: pages.as_slice(),
        }
        .render()?;
        create_dir_all(path.parent().ok_or("no parent")?)?;
        write(path, html)?;
    }

    for entry in WalkDir::new(args.out_dir.join("docs"))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(args.out_dir.as_path())?;
        let relative_path = relative_path.strip_prefix(Path::new("docs"))?;
        let new_path = args.out_dir.join(relative_path);
        create_dir_all(new_path.parent().ok_or("no parent")?)?;
        rename(path, &new_path)?;
    }
    remove_dir_all(args.out_dir.join("docs"))?;

    for entry in WalkDir::new(args.out_dir.as_path())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".html"))
    {}

    Ok(())
}
