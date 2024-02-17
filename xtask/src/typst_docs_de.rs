use std::marker::PhantomData;

use ecow::EcoString;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{serde_as, DeserializeAs, SerializeAs};
use typst_docs::{
    contributors, provide, urlify, Author, BodyModel, CategoryItem, CategoryModel, Commit,
    FuncModel, GroupModel, Html, OutlineItem, PageModel, ParamModel, Resolver, ShorthandsModel,
    StrParam, SymbolModel, SymbolsModel, TypeModel,
};

macro_rules! derive_deserialize_as {
    ($ret_name:ident, $def_name:ident) => {
        impl<'de> ::serde_with::DeserializeAs<'de, $ret_name> for $def_name {
            fn deserialize_as<D>(deserializer: D) -> Result<$ret_name, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                <$def_name>::deserialize(deserializer)
            }
        }
    };
}

type StaticStr = &'static str;
type StaticStrStaticSlice = &'static [&'static str];

struct StaticStrDef;
impl StaticStrDef {
    fn deserialize<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(Box::leak(s.into_boxed_str()))
    }
}
derive_deserialize_as!(StaticStr, StaticStrDef);

struct StaticStrStaticSliceDef;
impl StaticStrStaticSliceDef {
    fn deserialize<'de, D>(deserializer: D) -> Result<&'static [&'static str], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Vec<String> = Deserialize::deserialize(deserializer)?;
        let s: Vec<_> = s
            .into_iter()
            .map(|s| Box::leak(s.into_boxed_str()) as &str)
            .collect();
        Ok(Box::leak(s.into_boxed_slice()))
    }
}
derive_deserialize_as!(StaticStrStaticSlice, StaticStrStaticSliceDef);

struct HtmlDef;
impl HtmlDef {
    fn deserialize<'de, D>(deserializer: D) -> Result<Html, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(Html::new(s))
    }
}
derive_deserialize_as!(Html, HtmlDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "PageModel")]
pub struct PageModelDef {
    pub route: EcoString,
    pub title: EcoString,
    pub description: EcoString,
    #[serde_as(as = "Option<StaticStrDef>")]
    pub part: Option<StaticStr>,
    #[serde_as(as = "Vec<OutlineItemDef>")]
    pub outline: Vec<OutlineItem>,
    #[serde(with = "BodyModelDef")]
    pub body: BodyModel,
    #[serde_as(as = "Vec<PageModelDef>")]
    pub children: Vec<PageModel>,
}
derive_deserialize_as!(PageModel, PageModelDef);

#[serde_as]
#[derive(Deserialize)]
pub struct PageModelVecHelper(#[serde_as(as = "Vec<PageModelDef>")] pub Vec<PageModel>);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "OutlineItem")]
pub struct OutlineItemDef {
    pub id: EcoString,
    pub name: EcoString,
    #[serde_as(as = "Vec<OutlineItemDef>")]
    pub children: Vec<OutlineItem>,
}
derive_deserialize_as!(OutlineItem, OutlineItemDef);

#[derive(Deserialize)]
#[serde(
    remote = "BodyModel",
    rename_all = "camelCase",
    tag = "kind",
    content = "content"
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
derive_deserialize_as!(BodyModel, BodyModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "CategoryModel")]
pub struct CategoryModelDef {
    #[serde(with = "StaticStrDef")]
    pub name: StaticStr,
    #[serde(with = "StaticStrDef")]
    pub title: StaticStr,
    #[serde(with = "HtmlDef")]
    pub details: Html,
    #[serde_as(as = "Vec<CategoryItemDef>")]
    pub items: Vec<CategoryItem>,
    #[serde_as(as = "Option<ShorthandsModelDef>")]
    pub shorthands: Option<ShorthandsModel>,
}
derive_deserialize_as!(CategoryModel, CategoryModelDef);

#[derive(Deserialize)]
#[serde(remote = "CategoryItem")]
struct CategoryItemDef {
    pub name: EcoString,
    pub route: EcoString,
    pub oneliner: EcoString,
    pub code: bool,
}
derive_deserialize_as!(CategoryItem, CategoryItemDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "FuncModel")]
pub struct FuncModelDef {
    pub path: Vec<EcoString>,
    pub name: EcoString,
    #[serde(with = "StaticStrDef")]
    pub title: StaticStr,
    #[serde(with = "StaticStrStaticSliceDef")]
    pub keywords: StaticStrStaticSlice,
    #[serde(with = "StaticStrDef")]
    pub oneliner: StaticStr,
    pub element: bool,
    #[serde(with = "HtmlDef")]
    pub details: Html,
    #[serde_as(as = "Option<HtmlDef>")]
    pub example: Option<Html>,
    #[serde(rename = "self")]
    pub self_: bool,
    #[serde_as(as = "Vec<ParamModelDef>")]
    pub params: Vec<ParamModel>,
    #[serde_as(as = "Vec<StaticStrDef>")]
    pub returns: Vec<StaticStr>,
    #[serde_as(as = "Vec<FuncModelDef>")]
    pub scope: Vec<FuncModel>,
}
derive_deserialize_as!(FuncModel, FuncModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "ParamModel")]
pub struct ParamModelDef {
    #[serde(with = "StaticStrDef")]
    pub name: StaticStr,
    #[serde(with = "HtmlDef")]
    pub details: Html,
    #[serde_as(as = "Option<HtmlDef>")]
    pub example: Option<Html>,
    #[serde_as(as = "Vec<StaticStrDef>")]
    pub types: Vec<StaticStr>,
    #[serde_as(as = "Vec<StrParamDef>")]
    pub strings: Vec<StrParam>,
    #[serde_as(as = "Option<HtmlDef>")]
    pub default: Option<Html>,
    pub positional: bool,
    pub named: bool,
    pub required: bool,
    pub variadic: bool,
    pub settable: bool,
}
derive_deserialize_as!(ParamModel, ParamModelDef);

#[derive(Deserialize)]
#[serde(remote = "StrParam")]
pub struct StrParamDef {
    pub string: EcoString,
    #[serde(with = "HtmlDef")]
    pub details: Html,
}
derive_deserialize_as!(StrParam, StrParamDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "GroupModel")]
pub struct GroupModelDef {
    pub name: EcoString,
    pub title: EcoString,
    #[serde(with = "HtmlDef")]
    pub details: Html,
    #[serde_as(as = "Vec<FuncModelDef>")]
    pub functions: Vec<FuncModel>,
}
derive_deserialize_as!(GroupModel, GroupModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "TypeModel")]
pub struct TypeModelDef {
    #[serde(with = "StaticStrDef")]
    pub name: StaticStr,
    #[serde(with = "StaticStrDef")]
    pub title: StaticStr,
    #[serde(with = "StaticStrStaticSliceDef")]
    pub keywords: StaticStrStaticSlice,
    #[serde(with = "StaticStrDef")]
    pub oneliner: StaticStr,
    #[serde(with = "HtmlDef")]
    pub details: Html,
    #[serde_as(as = "Option<FuncModelDef>")]
    pub constructor: Option<FuncModel>,
    #[serde_as(as = "Vec<FuncModelDef>")]
    pub scope: Vec<FuncModel>,
}
derive_deserialize_as!(TypeModel, TypeModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "SymbolsModel")]
pub struct SymbolsModelDef {
    pub name: EcoString,
    pub title: EcoString,
    #[serde(with = "HtmlDef")]
    pub details: Html,
    #[serde_as(as = "Vec<SymbolModelDef>")]
    pub list: Vec<SymbolModel>,
}
derive_deserialize_as!(SymbolsModel, SymbolsModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", remote = "SymbolModel")]
pub struct SymbolModelDef {
    pub name: EcoString,
    pub codepoint: u32,
    pub accent: bool,
    pub unicode_name: Option<EcoString>,
    pub alternates: Vec<EcoString>,
    #[serde_as(as = "Option<StaticStrDef>")]
    pub markup_shorthand: Option<StaticStr>,
    #[serde_as(as = "Option<StaticStrDef>")]
    pub math_shorthand: Option<StaticStr>,
}
derive_deserialize_as!(SymbolModel, SymbolModelDef);

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "ShorthandsModel")]
pub struct ShorthandsModelDef {
    #[serde_as(as = "Vec<SymbolModelDef>")]
    pub markup: Vec<SymbolModel>,
    #[serde_as(as = "Vec<SymbolModelDef>")]
    pub math: Vec<SymbolModel>,
}
derive_deserialize_as!(ShorthandsModel, ShorthandsModelDef);
