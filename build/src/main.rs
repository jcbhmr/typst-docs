use askama::Template;
use lol_html::{element, rewrite_str, RewriteStrSettings};
use std::cell::Ref;
use std::env::var;
use std::rc::Rc;
use std::{
    error::Error,
    fs::{create_dir_all, write},
    path::Path,
};
use typst::{layout::Frame, model::Document, visualize::Color};
use typst_docs::{provide, Commit, FuncModel, Html, PageModel, Resolver, BodyModel, OutlineItem, SymbolsModel, TypeModel, Author, CategoryModel, GroupModel, CategoryItem,ParamModel,ShorthandsModel,StrParam,SymbolModel};
use typst_render::render;
use ecow::EcoString;

#[derive(Debug)]
struct MyResolver {
    out_dir: String,
}
impl Resolver for MyResolver {
    fn commits(&self, from: &str, to: &str) -> Vec<Commit> {
        eprintln!("commits({from}, {to})");
        vec![]
    }

    fn example(&self, hash: u128, source: Option<Html>, frames: &[Frame]) -> Html {
        eprintln!(
            "example(0x{hash:x}, {} chars, [Frame; {}])",
            source.as_ref().map(|html| html.as_str().len()).unwrap_or(0),
            frames.len()
        );
        let frame = frames.first().expect("No first frame in example");
        let pixmap = render(frame, 2.0, Color::WHITE);
        let hash = md5::compute(pixmap.data());
        let filename = format!("{hash:x}.png");
        let path = Path::new(&self.out_dir).join("assets/docs").join(&filename);
        create_dir_all(path.parent().expect("No parent")).expect("Failed to create dir");
        pixmap.save_png(path).expect("Failed to save pixmap");

        if let Some(source) = source {
            Html::new(format!(
                r#"
                <div class="previewed-code">
                    <pre>{}</pre>
                    <div class="preview">
                        <img src="{}" alt="Preview" width="480" height="190" />
                    </div>
                </div>
            "#,
                html_escape::encode_text(source.as_str()),
                format!("/assets/docs/{filename}")
            ))
        } else {
            Html::new("".to_string())
        }
    }

    fn image(&self, filename: &str, data: &[u8]) -> String {
        eprintln!("image({:?}, {} bytes)", filename, data.len());
        format!("/assets/docs/{filename}")
    }

    fn link(&self, link: &str) -> Option<String> {
        eprintln!("link({:?})", link);
        // Some(format!("/docs/{link}"))
        None
    }
}

#[derive(Template)]
#[template(path = "html.html")]
struct MyHtmlTemplate {
    pub route: EcoString,
    pub title: EcoString,
    pub description: EcoString,
    pub part: Option<&'static str>,
    pub outline: Vec<OutlineItem>,
    pub body: BodyModel,
}

#[derive(Template)]
#[template(path = "func.html")]
struct MyFuncTemplate {}

fn process_page(page: PageModel, out_dir: &str, base_url: &str) -> Result<(), Box<dyn Error>> {
    eprintln!("process_page({:?})", page.route);

    let route = if page.route.starts_with("/docs/") {
        &page.route["/docs".len()..]
    } else {
        &page.route
    };
    let route2 = if route.ends_with("/") {
        format!("{}/index.html", &page.route)
    } else {
        page.route.to_string()
    };
    let path =
        Path::new(&out_dir).join(route2.strip_prefix("/").expect("No prefix"));

    let html = MyHtmlTemplate {
        route: page.route,
        title: page.title,
        description: page.description,
        part: page.part,
        outline: page.outline,
        body: page.body,
    }.render()?;

    let html = rewrite_str(
        &html,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("[href]", |el| {
                    let href = el.get_attribute("href").unwrap();
                    if href.starts_with("/docs/") {
                        el.set_attribute(
                            "href",
                            &format!("{}{}", base_url, &href["/docs/".len()..]),
                        )?;
                    } else if href.starts_with("/") {
                        el.set_attribute(
                            "href",
                            &format!("{}{}", base_url, &href[1..]),
                        )?;
                    }
                    Ok(())
                }),
                element!("[src]", |el| {
                    let src = el.get_attribute("src").unwrap();
                    if src.starts_with("/") {
                        el.set_attribute(
                            "src",
                            &format!("{}{}", base_url, &src[1..]),
                        )?;
                    }
                    Ok(())
                }),
            ],
            ..Default::default()
        },
    )?;

    create_dir_all(path.parent().expect("No parent")).expect("Failed to create dir");
    write(&path, html)?;

    for child in page.children {
        process_page(child, out_dir, base_url)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = var("OUT_DIR").unwrap_or_else(|_| "_site".to_string());
    let base_url = var("BASE_URL").unwrap_or_else(|_| "/".to_string());
    let base_url = if base_url.ends_with("/") {
        base_url
    } else {
        format!("{}/", base_url)
    };

    let pages = provide(&MyResolver {
        out_dir: out_dir.clone(),
    });
    for page in pages {
        process_page(page, &out_dir, &base_url)?;
    }

    Ok(())
}
