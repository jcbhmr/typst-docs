mod html;
mod macros;
mod static_str;

pub use self::html::*;

use self::macros::deserialize_as_impl;
use self::static_str::{StaticStr, StaticStrDef, StaticStrSlice, StaticStrSliceDef};
use ecow::EcoString;
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, DeserializeAs, SerializeAs};
use typst_docs::{
    BodyModel, CategoryItem, CategoryModel, Commit, FuncModel, GroupModel, Html, OutlineItem,
    PageModel, ParamModel, ShorthandsModel, StrParam, SymbolModel, SymbolsModel, TypeModel,
};

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "PageModel")]
pub struct PageModelDef {
    route: EcoString,
    title: EcoString,
    description: EcoString,
    #[serde_as(as = "Option<StaticStrDef>")]
    part: Option<StaticStr>,
    #[serde_as(as = "Vec<OutlineItemDef>")]
    outline: Vec<OutlineItem>,
    #[serde(with = "BodyModelDef")]
    body: BodyModel,
    #[serde_as(as = "Vec<PageModelDef>")]
    children: Vec<PageModel>,
}
deserialize_as_impl!(PageModel, PageModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "OutlineItem")]
pub struct OutlineItemDef {
    id: EcoString,
    name: EcoString,
    #[serde_as(as = "Vec<OutlineItemDef>")]
    children: Vec<OutlineItem>,
}
deserialize_as_impl!(OutlineItem, OutlineItemDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(
    rename_all = "camelCase",
    tag = "kind",
    content = "content",
    remote = "BodyModel"
)]
pub enum BodyModelDef {
    #[serde(with = "HtmlDef")]
    Html(Html),
    #[serde(with = "CategoryModelDef")]
    Category(CategoryModel),
    #[serde(with = "FuncModelDef")]
    Func(FuncModel),
    #[serde(with = "GroupModelDef")]
    Group(GroupModel),
    #[serde(with = "TypeModelDef")]
    Type(TypeModel),
    #[serde(with = "SymbolsModelDef")]
    Symbols(SymbolsModel),
    #[serde(with = "HtmlDef")]
    Packages(Html),
}
deserialize_as_impl!(BodyModel, BodyModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "CategoryModel")]
pub struct CategoryModelDef {
    #[serde_as(as = "StaticStrDef")]
    name: StaticStr,
    #[serde_as(as = "StaticStrDef")]
    title: StaticStr,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde_as(as = "Vec<CategoryItemDef>")]
    items: Vec<CategoryItem>,
    #[serde_as(as = "Option<ShorthandsModelDef>")]
    shorthands: Option<ShorthandsModel>,
}
deserialize_as_impl!(CategoryModel, CategoryModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "CategoryItem")]
pub struct CategoryItemDef {
    name: EcoString,
    route: EcoString,
    oneliner: EcoString,
    code: bool,
}
deserialize_as_impl!(CategoryItem, CategoryItemDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "FuncModel")]
pub struct FuncModelDef {
    path: Vec<EcoString>,
    name: EcoString,
    #[serde_as(as = "StaticStrDef")]
    title: StaticStr,
    #[serde_as(as = "StaticStrSliceDef")]
    keywords: StaticStrSlice,
    #[serde_as(as = "StaticStrDef")]
    oneliner: StaticStr,
    element: bool,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde_as(as = "Option<HtmlDef>")]
    example: Option<Html>,
    #[serde(rename = "self")]
    self_: bool,
    #[serde_as(as = "Vec<ParamModelDef>")]
    params: Vec<ParamModel>,
    #[serde_as(as = "Vec<StaticStrDef>")]
    returns: Vec<StaticStr>,
    #[serde_as(as = "Vec<FuncModelDef>")]
    scope: Vec<FuncModel>,
}
deserialize_as_impl!(FuncModel, FuncModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "ParamModel")]
pub struct ParamModelDef {
    #[serde_as(as = "StaticStrDef")]
    name: StaticStr,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde_as(as = "Option<HtmlDef>")]
    example: Option<Html>,
    #[serde_as(as = "Vec<StaticStrDef>")]
    types: Vec<StaticStr>,
    #[serde_as(as = "Vec<StrParamDef>")]
    strings: Vec<StrParam>,
    #[serde_as(as = "Option<HtmlDef>")]
    default: Option<Html>,
    positional: bool,
    named: bool,
    required: bool,
    variadic: bool,
    settable: bool,
}
deserialize_as_impl!(ParamModel, ParamModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "StrParam")]
pub struct StrParamDef {
    string: EcoString,
    #[serde(with = "HtmlDef")]
    details: Html,
}
deserialize_as_impl!(StrParam, StrParamDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "GroupModel")]
pub struct GroupModelDef {
    name: EcoString,
    title: EcoString,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde_as(as = "Vec<FuncModelDef>")]
    functions: Vec<FuncModel>,
}
deserialize_as_impl!(GroupModel, GroupModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "TypeModel")]
pub struct TypeModelDef {
    #[serde_as(as = "StaticStrDef")]
    name: StaticStr,
    #[serde_as(as = "StaticStrDef")]
    title: StaticStr,
    #[serde_as(as = "StaticStrSliceDef")]
    keywords: StaticStrSlice,
    #[serde_as(as = "StaticStrDef")]
    oneliner: StaticStr,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde_as(as = "Option<FuncModelDef>")]
    constructor: Option<FuncModel>,
    #[serde_as(as = "Vec<FuncModelDef>")]
    scope: Vec<FuncModel>,
}
deserialize_as_impl!(TypeModel, TypeModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "SymbolsModel")]
pub struct SymbolsModelDef {
    name: EcoString,
    title: EcoString,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde_as(as = "Vec<SymbolModelDef>")]
    list: Vec<SymbolModel>,
}
deserialize_as_impl!(SymbolsModel, SymbolsModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "SymbolModel")]
#[serde(rename_all = "camelCase")]
pub struct SymbolModelDef {
    name: EcoString,
    codepoint: u32,
    accent: bool,
    unicode_name: Option<EcoString>,
    alternates: Vec<EcoString>,
    #[serde_as(as = "Option<StaticStrDef>")]
    markup_shorthand: Option<StaticStr>,
    #[serde_as(as = "Option<StaticStrDef>")]
    math_shorthand: Option<StaticStr>,
}
deserialize_as_impl!(SymbolModel, SymbolModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "ShorthandsModel")]
pub struct ShorthandsModelDef {
    #[serde_as(as = "Vec<SymbolModelDef>")]
    markup: Vec<SymbolModel>,
    #[serde_as(as = "Vec<SymbolModelDef>")]
    math: Vec<SymbolModel>,
}
deserialize_as_impl!(ShorthandsModel, ShorthandsModelDef);
