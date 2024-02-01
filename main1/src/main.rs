use dataurl;
use html_escape;
use lazy_static::lazy_static;
use std::env;
use std::fs;
use typst;
use typst_docs;
use typst_render;

lazy_static! {
    static ref DOCS_DIR: String = env::var("DOCS_DIR").unwrap();
    static ref IMAGE_PREFIX: String = env::var("IMAGE_PREFIX").unwrap();
}

struct MyResolver;
impl typst_docs::Resolver for MyResolver {
    fn commits(&self, from: &str, to: &str) -> Vec<typst_docs::Commit> {
        vec![]
    }
    fn example(
        &self,
        hash: u128,
        source: Option<typst_docs::Html>,
        document: &typst::model::Document,
    ) -> typst_docs::Html {
        let frame = &document.pages.first().unwrap().frame;
        let pixmap = typst_render::render(frame, 2.0, typst::visualize::Color::WHITE);
        if let Some(source) = source {
            let mut data_url = dataurl::DataUrl::new();
            data_url.set_media_type(Some("image/png".into()));
            data_url.set_data(pixmap.data());
            typst_docs::Html::new(format!(
                r#"<div class="previewed-code">
                    <pre>{}</pre>
                    <div class="preview">
                        <img src="{}" alt="Preview" width="480" height="190" />
                    </div>
                </div>"#,
                source.as_str(),
                data_url.to_string(),
            ))
        } else {
            typst_docs::Html::new(String::new())
        }
    }
    fn image(&self, filename: &str, data: &[u8]) -> String {
        format!("{}/{}", *IMAGE_PREFIX, filename)
    }
    fn link(&self, link: &str) -> Option<String> {
        None
    }
}

pub fn main() {
    let pages = typst_docs::provide(&MyResolver);
}
