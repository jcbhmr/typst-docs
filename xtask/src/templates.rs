use std::marker::PhantomData;

use askama::Template;
use ecow::EcoString;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{serde_as, DeserializeAs, SerializeAs};
use typst_docs::{
    contributors, provide, urlify, Author, BodyModel, CategoryItem, CategoryModel, Commit,
    FuncModel, GroupModel, Html, OutlineItem, PageModel, ParamModel, Resolver, ShorthandsModel,
    StrParam, SymbolModel, SymbolsModel, TypeModel,
};

#[derive(Template)]
#[template(path = "category.md")]
pub struct CategoryTemplate<'a> {
    pub page: &'a PageModel,
    pub category: &'a CategoryModel,
}

#[derive(Template)]
#[template(path = "func.md")]
pub struct FuncTemplate<'a> {
    pub page: &'a PageModel,
    pub func: &'a FuncModel,
}

#[derive(Template)]
#[template(path = "group.md")]
pub struct GroupTemplate<'a> {
    pub page: &'a PageModel,
    pub group: &'a GroupModel,
}

#[derive(Template)]
#[template(path = "html.md")]
pub struct HtmlTemplate<'a> {
    pub page: &'a PageModel,
    pub html: &'a Html,
}

#[derive(Template)]
#[template(path = "packages.md")]
pub struct PackagesTemplate<'a> {
    pub page: &'a PageModel,
    pub packages: &'a Html,
}

#[derive(Template)]
#[template(path = "symbols.md")]
pub struct SymbolsTemplate<'a> {
    pub page: &'a PageModel,
    pub symbols: &'a SymbolsModel,
}

#[derive(Template)]
#[template(path = "type.md")]
pub struct TypeTemplate<'a> {
    pub page: &'a PageModel,
    pub type_: &'a TypeModel,
}

#[derive(Template)]
#[template(path = ".vitepress/config/index.ts", escape = "none")]
pub struct VitepressConfigIndexTemplate<'a> {
    pub locales: &'a [(&'a str, &'a str)],
}

#[derive(Template)]
#[template(path = ".vitepress/config/locale.ts", escape = "none")]
pub struct VitepressConfigLocaleTemplate<'a> {
    pub label: &'a str,
    pub locale: &'a str,
    pub title: &'a str,
    pub description: &'a str,
}

#[derive(Template)]
#[template(path = "index.md")]
pub struct IndexTemplate<'a> {
    pub locales: &'a [(&'a str, &'a str)],
}
