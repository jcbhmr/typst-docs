use std::error::Error;
use typst::model::Document;
use typst_docs::{provide, Commit, Html, Resolver};

struct MyResolver;
impl Resolver for MyResolver {
    fn commits(&self, from: &str, to: &str) -> Vec<Commit> {
        vec![]
    }

    fn example(&self, hash: u128, source: Option<Html>, document: &Document) -> Html {
        Html::new("".to_string())
    }

    fn image(&self, filename: &str, data: &[u8]) -> String {
        format!("/assets/docs/{}", filename)
    }

    fn link(&self, link: &str) -> Option<String> {
        Some(format!("/docs/{}", link))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let pages = provide(&MyResolver);

    for page in pages {
        println!("{:?}", page);
    }

    Ok(())
}
