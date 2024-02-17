mod typst_docs_de;
mod templates;

use std::{
    env::{args, set_current_dir},
    error::Error,
    fs::{create_dir_all, remove_dir_all, rename, write},
    panic::catch_unwind,
    path::Path,
};

use askama::Template;
use ecow::EcoString;
use serde::Deserialize;
use serde_json::Deserializer;
use typst_docs::{
    provide, BodyModel, CategoryModel, Commit, FuncModel, GroupModel, Html, PageModel, Resolver,
    SymbolsModel, TypeModel,
};
use xshell::{cmd, Shell};
use include_dir::{include_dir, Dir};
// use lol_html::{element, rewrite_str, RewriteStrSettings};

use crate::templates::{IndexTemplate, VitepressConfigIndexTemplate, VitepressConfigLocaleTemplate};
use crate::typst_docs_de::{PageModelDef, PageModelVecHelper};
use crate::templates::{CategoryTemplate,FuncTemplate,GroupTemplate,HtmlTemplate,PackagesTemplate,SymbolsTemplate,TypeTemplate};

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");
const SRC_LOCALES: &[&str] = &["es", "zh"];

fn build() -> Result<(), Box<dyn Error>> {
    let sh = Shell::new()?;
    remove_dir_all("out")?;
    for locale in SRC_LOCALES {
        eprintln!("Building {locale} docs");

        let manifest_path = format!("crates/typst-{locale}/crates/typst-docs/Cargo.toml");
        let out_dir = Path::new("out").join(&locale);
        let public_dir = Path::new("out").join("public");

        let stdout = cmd!(
            sh,
            "cargo run --manifest-path {manifest_path} --features=cli --target-dir=target -- --out-dir {public_dir}"
        )
        .read()?;
        print!("{stdout}");

        let root_pages: Vec<PageModel> =
            serde_json::from_str(&stdout).map(|PageModelVecHelper(v)| v)?;
        eprintln!("Parsed data for {} root pages", root_pages.len());

        fn all_pages_helper<'a>(page: &'a PageModel, mut all_pages: &mut Vec<&'a PageModel>) {
            all_pages.push(page);
            for page in &page.children {
                all_pages_helper(page, &mut all_pages);
            }
        }
        let mut all_pages: Vec<&PageModel> = Vec::new();
        for root_page in &root_pages {
            all_pages_helper(root_page, &mut all_pages);
        }
        eprintln!(
            "Crawled root pages and found data for {} total pages",
            all_pages.len()
        );

        for page in &all_pages {
            let mut path = out_dir.clone();
            let mut route_path = page.route.to_string();
            if route_path.ends_with("/") {
                route_path.push_str("index.md");
            }
            if route_path.starts_with("/") {
                route_path.remove(0);
            }
            path.push(route_path);

            let html = match &page.body {
                BodyModel::Category(category) => CategoryTemplate { page, category }.render()?,
                BodyModel::Func(func) => FuncTemplate { page, func }.render()?,
                BodyModel::Group(group) => GroupTemplate { page, group }.render()?,
                BodyModel::Html(html) => HtmlTemplate { page, html }.render()?,
                BodyModel::Packages(packages) => PackagesTemplate { page, packages }.render()?,
                BodyModel::Symbols(symbols) => SymbolsTemplate { page, symbols }.render()?,
                BodyModel::Type(type_) => TypeTemplate { page, type_ }.render()?,
            };
            eprintln!(
                "Generated {} chars of HTML for {:?}",
                html.len(),
                &page.route
            );

            create_dir_all(path.parent().ok_or("no parent")?)?;
            write(&path, html)?;
            eprintln!("Created {:?}", &path);
        }
        eprintln!("Created all Markdown files for {locale}");

        let ts = VitepressConfigLocaleTemplate {
            description: "",
            label: &locale,
            locale: &locale,
            title: "Typst Documentation",
            root_pages: &root_pages,
        }.render()?;
        let path = Path::new("out").join(".vitepress").join("config").join(format!("{locale}.ts"));
        create_dir_all(path.parent().ok_or("no parent")?)?;
        write(&path, ts)?;
        eprintln!("Created {locale} config file {path:?}");
    }

    let ts = VitepressConfigIndexTemplate {
        locales: &[
            ("es", "es"),
            ("zh", "zh"),
        ],
    }.render()?;
    let path = Path::new("out").join(".vitepress").join("config").join("index.ts");
    create_dir_all(path.parent().ok_or("no parent")?)?;
    write(&path, ts)?;
    eprintln!("Created index config file {path:?}");

    let index = IndexTemplate {
        locales: &[
            ("es", "es"),
            ("zh", "zh"),
        ],
    }.render()?;
    let path = Path::new("out").join("index.md");
    create_dir_all(path.parent().ok_or("no parent")?)?;
    write(&path, index)?;
    eprintln!("Created index file {path:?}");
    
    create_dir_all("out")?;
    ASSETS.extract("out")?;
    eprintln!("Extracted static assets to out");

    let sh = Shell::new()?;
    create_dir_all("out")?;
    sh.change_dir("out");
    cmd!(sh, "npm install").run()?;
    cmd!(sh, "npm run build").run()?;
    
    remove_dir_all("_site")?;
    rename("out/.vitepress/dist", "_site")?;
    eprintln!("Moved final site to '_site'");

    Ok(())
}

fn diff() -> Result<(), Box<dyn Error>> {
    let sh = Shell::new()?;
    create_dir_all("patches")?;
    for locale in SRC_LOCALES {
        let patch_file = format!("patches/typst-{locale}.patch");
        cmd!(sh, "git -C crates/typst-{locale} reset").run()?;
        cmd!(sh, "git -C crates/typst-{locale} add -AN").run()?;
        let stdout = cmd!(sh, "git -C crates/typst-{locale} diff --binary").read()?;
        write(&patch_file, stdout)?;
        eprintln!("Collected stdout and wrote to {patch_file:?}");
    }
    Ok(())
}

fn apply() -> Result<(), Box<dyn Error>> {
    let sh = Shell::new()?;
    for locale in SRC_LOCALES {
        let patch_file = format!("patches/typst-{locale}.patch");
        cmd!(sh, "git -C crates/typst-{locale} add -AN").run()?;
        cmd!(sh, "git -C crates/typst-{locale} reset --hard").run()?;
        cmd!(sh, "git -C crates/typst-{locale} apply ../../{patch_file}").run()?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    match args().nth(1).ok_or("no task specified")?.as_str() {
        "build" => build(),
        "diff" => diff(),
        "apply" => apply(),
        _ => Err("task not found".into()),
    }
}
