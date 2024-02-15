use askama::Template;
use clap::Parser;
use ecow::EcoString;
use std::{
    collections::HashMap, error::Error, fs::{create_dir_all, remove_dir_all, rename, write}, panic::catch_unwind, path::{Path, PathBuf}
};
use typst::{layout::Frame, visualize::Color};
use typst_docs::{provide, BodyModel, Html, PageModel, Resolver};
use typst_render::render;
use walkdir::WalkDir;
use include_dir::{include_dir, Dir};

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

struct MyResolver<'a> {
    out_dir: &'a Path,
    assets_dir: &'a Path,
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
        frames: &[Frame],
    ) -> typst_docs::Html {
        eprintln!(
            "example(0x{hash:x}, {:?} chars, &[Frame; {}])",
            source.as_ref().map(|s| s.as_str().len()),
            frames.len()
        );

        Html::new("".to_string())
    }
    fn image(&self, filename: &str, data: &[u8]) -> String {
        eprintln!("image({filename}, {} bytes)", data.len());

        let path = self.assets_dir.join(filename);
        create_dir_all(path.parent().expect("parent")).expect("create dir");
        write(path, data).expect("write image");

        format!("/assets/{filename}")
    }
    fn link(&self, link: &str) -> Option<String> {
        eprintln!("link({link})");
        None
    }
}

#[derive(Template)]
#[template(path = "html.html")]
struct HtmlTemplate<'a> {
    lang: &'a str,
    page: &'a PageModel,
    breadcrumbs: &'a [&'a PageModel],
    prev: Option<&'a PageModel>,
    next: Option<&'a PageModel>,
    pages: &'a [&'a PageModel],
    html: &'a Html,
}

#[derive(Template)]
#[template(path = "func.html")]
struct FuncTemplate {}

#[derive(Template)]
#[template(path = "category.html")]
struct CategoryTemplate {}

#[derive(Template)]
#[template(path = "group.html")]
struct GroupTemplate {}

#[derive(Template)]
#[template(path = "type.html")]
struct TypeTemplate {}

#[derive(Template)]
#[template(path = "symbols.html")]
struct SymbolsTemplate {}

#[derive(Template)]
#[template(path = "packages.html")]
struct PackagesTemplate<'a> {
    lang: &'a str,
    page: &'a PageModel,
    breadcrumbs: &'a [&'a PageModel],
    prev: Option<&'a PageModel>,
    next: Option<&'a PageModel>,
    pages: &'a [&'a PageModel],
    packages: &'a Html,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Where to put the final site HTML files
    #[arg(short, long, default_value = "_site")]
    out_dir: PathBuf,

    /// Where to put the assets (images, fonts, CSS, etc.)
    #[arg(short, long, default_value = "_site/assets")]
    assets_dir: PathBuf,

    /// The base URL of the site
    #[arg(short, long, default_value = "/")]
    base_url: String,

    /// The BCP-47 language tag to go in the `lang` attribute of the `<html>` tag
    #[arg(short, long, default_value = "en")]
    lang: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut base_url = args.base_url.clone();
    if !base_url.ends_with("/") {
        base_url.push_str("/");
    }

    let own_pages = provide(&MyResolver {
        out_dir: args.out_dir.as_path(),
        assets_dir: args.assets_dir.as_path(),
    });
    let pages: Vec<_> = own_pages.iter().collect();

    fn pages_flat_helper<'a>(page: &'a PageModel, pages: &mut Vec<&'a PageModel>) {
        pages.push(page);
        pages.extend(page.children.iter());
    }
    let mut all_pages: Vec<&PageModel> = Vec::new();
    for page in &pages {
        pages_flat_helper(page, &mut all_pages);
    }

    fn page_parents_helper<'a>(
        pages: Vec<&'a PageModel>,
        parent: &'a PageModel,
        page_parents: &mut HashMap<EcoString, &'a PageModel>,
    ) {
        for page in pages {
            page_parents.insert(page.route.clone(), parent);
            let children = page.children.iter().collect::<Vec<_>>();
            page_parents_helper(children, page, page_parents);
        }
    }
    let mut page_parents: HashMap<EcoString, &PageModel> = HashMap::new();
    for page in &pages {
        let children = page.children.iter().collect::<Vec<_>>();
        page_parents_helper(children, page, &mut page_parents);
    }

    let mut page_breadcrumbs: HashMap<EcoString, Vec<&PageModel>> = HashMap::new();
    for page in &all_pages {
        let mut breadcrumbs: Vec<&PageModel> = vec![];
        let mut current = page;
        while let Some(parent) = page_parents.get(&current.route) {
            breadcrumbs.push(current);
            current = parent;
        }
        breadcrumbs.push(current);
        breadcrumbs.reverse();
        page_breadcrumbs.insert(page.route.clone(), breadcrumbs);
    }

    let mut page_prev: HashMap<EcoString, &PageModel> = HashMap::new();
    let mut page_next: HashMap<EcoString, &PageModel> = HashMap::new();
    for (i, page) in all_pages.iter().enumerate() {
        if i > 0 {
            page_prev.insert(page.route.clone(), all_pages[i - 1]);
        }
        if i < all_pages.len() - 1 {
            page_next.insert(page.route.clone(), all_pages[i + 1]);
        }
    }

    for page in &all_pages {
        let breadcrumbs = page_breadcrumbs.get(&page.route).expect("no breadcrumbs");
        let prev = page_prev.get(&page.route).copied();
        let next = page_next.get(&page.route).copied();

        let path = args.out_dir.clone();
        let mut route = page.route.to_string();
        if route.ends_with("/") {
            route.push_str("index.html");
        }
        let mut route = route.strip_prefix("/docs/").unwrap_or(&route).to_string();
        if route.starts_with("/") {
            route.remove(0);
        }
        let path = path.join(route);

        let html = match page.body {
            BodyModel::Html(ref html) => {
                HtmlTemplate {
                    lang: args.lang.as_str(),
                    page,
                    breadcrumbs: breadcrumbs.as_slice(),
                    prev,
                    next,
                    pages: all_pages.as_slice(),
                    html,
                }.render()?
            },
            BodyModel::Category(ref category) => {
                CategoryTemplate{}.render()?
            },
            BodyModel::Func(ref func) => {
                FuncTemplate{}.render()?
            },
            BodyModel::Group(ref group) => {
                GroupTemplate{}.render()?
            },
            BodyModel::Type(ref type_) => {
                TypeTemplate{}.render()?
            },
            BodyModel::Symbols(ref symbols) => {
                SymbolsTemplate{}.render()?
            },
            BodyModel::Packages(ref packages) => {
                PackagesTemplate{
                    lang: args.lang.as_str(),
                    page,
                    breadcrumbs: breadcrumbs.as_slice(),
                    prev,
                    next,
                    pages: all_pages.as_slice(),
                    packages,
                }.render()?
            },
        };

        create_dir_all(path.parent().ok_or("no parent")?)?;
        write(path, html)?;
    }

    create_dir_all(args.assets_dir.as_path())?;
    ASSETS.extract(args.assets_dir)?;

    Ok(())
}
