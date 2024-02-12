use std::{error::Error, f64::consts::E, fs::{create_dir_all, write}, path::Path};
use typst::{layout::Frame, model::Document, visualize::Color};
use typst_docs::{provide, Commit, FuncModel, Html, PageModel, Resolver};
use lol_html::{element, rewrite_str, RewriteStrSettings};
use askama::Template;
use typst_render::render;
use std::env::var;

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
        eprintln!("example({hash:x}, {} chars, [Frame; {}])", source.as_ref().map(|html| html.as_str().len()).unwrap_or(0), frames.len());
        let frame = frames.first().expect("No first frame in example");
        let pixmap = render(frame, 2.0, Color::WHITE);
        let hash = md5::compute(pixmap.data());
        let filename = format!("{hash:x}.png");
        let path = Path::new(&self.out_dir).join("assets/docs").join(&filename);
        create_dir_all(path.parent().expect("No parent")).expect("Failed to create dir");
        pixmap.save_png(path).expect("Failed to save pixmap");

        if let Some(source) = source {
            Html::new(format!(r#"
                <div class="previewed-code">
                    <pre>{}</pre>
                    <div class="preview">
                        <img src="{}" alt="Preview" width="480" height="190" />
                    </div>
                </div>
            "#, html_escape::encode_text(source.as_str()), format!("/assets/docs/{filename}")))
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
}

#[derive(Template)]
#[template(path = "func.html")]
struct MyFuncTemplate {
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = var("OUT_DIR").unwrap_or_else(|_| "_site".to_string());
    let base_url = var("BASE_URL").unwrap_or_else(|_| "/".to_string());
    let base_url_with_trailing_slash = if base_url.ends_with("/") { base_url } else { format!("{}/", base_url) };

    let pages = provide(&MyResolver { out_dir: out_dir.clone() });
    let pages_flat = pages.iter().flat_map(|page| {
        let mut pages = vec![page];
        pages.extend(page.children.iter());
        pages
    }).collect::<Vec<_>>();

    for page in pages_flat {
        let route_with_index_html = if page.route.ends_with("/") { format!("{}/index.html", &page.route) } else { page.route.to_string() };
        let path = Path::new(&out_dir).join(route_with_index_html.strip_prefix("/").expect("No prefix"));
        let html = MyHtmlTemplate {}.render()?;
        let html = rewrite_str(&html, RewriteStrSettings {
            element_content_handlers: vec![
                element!("[href]", |el| {
                    let href = el.get_attribute("href").unwrap();
                    if href.starts_with("/") {
                        el.set_attribute("href", &format!("{}{}", base_url_with_trailing_slash, &href[1..]))?;
                    }
                    Ok(())
                }),
                element!("[src]", |el| {
                    let src = el.get_attribute("src").unwrap();
                    if src.starts_with("/") {
                        el.set_attribute("src", &format!("{}{}", base_url_with_trailing_slash, &src[1..]))?;
                    }
                    Ok(())
                }),
                ],
            ..Default::default()
        })?;
        create_dir_all(path.parent().expect("No parent")).expect("Failed to create dir");
        write(&path, html)?;
    }

    Ok(())
}
