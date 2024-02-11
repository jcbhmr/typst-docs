use std::error::Error;
use typst_docs::{provide, Resolver, Commit};
use typst::model::Document;

struct MyResolver;
impl Resolver for MyResolver {
    fn commits(&self, from: &str, to: &str) -> Vec<typst_docs::Commit> {
        todo!()
    }
    fn example(
        &self,
        hash: u128,
        source: Option<typst_docs::Html>,
        document: &typst::model::Document,
    ) -> typst_docs::Html {
        todo!()
        document
    }
    fn image(&self, filename: &str, data: &[u8]) -> String {
        todo!()
    }
    fn link(&self, link: &str) -> Option<String> {
        todo!()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let pages = provide(&MyResolver);
    Ok(())
}
