mod static_str;
mod html;

pub use html::*;

use typst_docs::{PageModel, OutlineItem, BodyModel, Html, CategoryModel, FuncModel, GroupModel, TypeModel, SymbolsModel, CategoryItem, ShorthandsModel, StrParam, ParamModel, SymbolModel, };
use serde::{Deserialize, Deserializer};
use ecow::EcoString;
use static_str::{StaticStrSlice,StaticStr, StaticStrDef, StaticStrSliceDef};
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "PageModel")]
pub struct PageModelDef {
    route: EcoString,
    title: EcoString,
    description: EcoString,
    #[serde_as(as = "Option<StaticStrDef>")]
    part: Option<StaticStr>,
    #[serde(deserialize_with = "OutlineItemDef::deserialize_vec")]
    outline: Vec<OutlineItem>,
    #[serde(with = "BodyModelDef")]
    body: BodyModel,
    #[serde(deserialize_with = "PageModelDef::deserialize_vec")]
    children: Vec<PageModel>,
}

#[derive(Deserialize)]
struct PageModelWrapper(#[serde(with = "PageModelDef")] PageModel);

impl PageModelDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<PageModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|PageModelWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<PageModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|PageModelWrapper(s)| s))
    }
}

#[derive(Deserialize)]
#[serde(remote = "OutlineItem")]
pub struct OutlineItemDef {
    id: EcoString,
    name: EcoString,
    #[serde(deserialize_with = "OutlineItemDef::deserialize_vec")]
    children: Vec<OutlineItem>,
}

#[derive(Deserialize)]
struct OutlineItemWrapper(#[serde(with = "OutlineItemDef")] OutlineItem);

impl OutlineItemDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<OutlineItem>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|OutlineItemWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<OutlineItem>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|OutlineItemWrapper(s)| s))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "content", remote = "BodyModel")]
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

#[derive(Deserialize)]
struct BodyModelWrapper(#[serde(with = "BodyModelDef")] BodyModel);

impl BodyModelDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<BodyModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|BodyModelWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<BodyModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|BodyModelWrapper(s)| s))
    }
}

#[derive(Deserialize)]
#[serde(remote = "CategoryModel")]
pub struct CategoryModelDef {
    #[serde(deserialize_with = "deserialize_static_str")]
    name: StaticStr,
    #[serde(deserialize_with = "deserialize_static_str")]
    title: StaticStr,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde(deserialize_with = "CategoryItemDef::deserialize_vec")]
    items: Vec<CategoryItem>,
    #[serde(deserialize_with = "ShorthandsModelDef::deserialize_option")]
    shorthands: Option<ShorthandsModel>,
}

#[derive(Deserialize)]
struct CategoryModelWrapper(#[serde(with = "CategoryModelDef")] CategoryModel);

impl CategoryModelDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<CategoryModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|CategoryModelWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<CategoryModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|CategoryModelWrapper(s)| s))
    }
}

#[derive(Deserialize)]
#[serde(remote = "CategoryItem")]
pub struct CategoryItemDef {
    name: EcoString,
    route: EcoString,
    oneliner: EcoString,
    code: bool,
}

#[derive(Deserialize)]
struct CategoryItemWrapper(#[serde(with = "CategoryItemDef")] CategoryItem);

impl CategoryItemDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<CategoryItem>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|CategoryItemWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<CategoryItem>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|CategoryItemWrapper(s)| s))
    }
}

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "FuncModel")]
pub struct FuncModelDef {
    path: Vec<EcoString>,
    name: EcoString,
    #[serde(deserialize_with = "deserialize_static_str")]
    title: StaticStr,
    #[serde(deserialize_with = "deserialize_static_str_slice")]
    keywords: StaticStrSlice,
    #[serde(deserialize_with = "deserialize_static_str")]
    oneliner: StaticStr,
    element: bool,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde_as(as = "Option<HtmlDef>")]
    example: Option<Html>,
    #[serde(rename = "self")]
    self_: bool,
    #[serde(deserialize_with = "ParamModelDef::deserialize_vec")]
    params: Vec<ParamModel>,
    #[serde(deserialize_with = "deserialize_static_str_vec")]
    returns: Vec<StaticStr>,
    #[serde(deserialize_with = "FuncModelDef::deserialize_vec")]
    scope: Vec<FuncModel>,
}

#[derive(Deserialize)]
struct FuncModelWrapper(#[serde(with = "FuncModelDef")] FuncModel);

impl FuncModelDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<FuncModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|FuncModelWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<FuncModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|FuncModelWrapper(s)| s))
    }
}

#[serde_as]
#[derive(Deserialize)]
#[serde(remote = "ParamModel")]
pub struct ParamModelDef {
    #[serde(deserialize_with = "deserialize_static_str")]
    name: StaticStr,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde_as(as = "Option<HtmlDef>")]
    example: Option<Html>,
    #[serde(deserialize_with = "deserialize_static_str_vec")]
    types: Vec<StaticStr>,
    #[serde(deserialize_with = "StrParamDef::deserialize_vec")]
    strings: Vec<StrParam>,
    #[serde_as(as = "Option<HtmlDef>")]
    default: Option<Html>,
    positional: bool,
    named: bool,
    required: bool,
    variadic: bool,
    settable: bool,
}

#[derive(Deserialize)]
struct ParamModelWrapper(#[serde(with = "ParamModelDef")] ParamModel);

impl ParamModelDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<ParamModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|ParamModelWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<ParamModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|ParamModelWrapper(s)| s))
    }
}

#[derive(Deserialize)]
#[serde(remote = "StrParam")]
pub struct StrParamDef {
    string: EcoString,
    #[serde(with = "HtmlDef")]
    details: Html,
}

#[derive(Deserialize)]
struct StrParamWrapper(#[serde(with = "StrParamDef")] StrParam);

impl StrParamDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<StrParam>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|StrParamWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<StrParam>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|StrParamWrapper(s)| s))
    }
}

#[derive(Deserialize)]
#[serde(remote = "GroupModel")]
pub struct GroupModelDef {
    name: EcoString,
    title: EcoString,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde(deserialize_with = "FuncModelDef::deserialize_vec")]
    functions: Vec<FuncModel>,
}

#[derive(Deserialize)]
struct GroupModelWrapper(#[serde(with = "GroupModelDef")] GroupModel);

impl GroupModelDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<GroupModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|GroupModelWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<GroupModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|GroupModelWrapper(s)| s))
    }
}

#[derive(Deserialize)]
#[serde(remote = "TypeModel")]
pub struct TypeModelDef {
    #[serde(deserialize_with = "deserialize_static_str")]
    name: StaticStr,
    #[serde(deserialize_with = "deserialize_static_str")]
    title: StaticStr,
    #[serde(deserialize_with = "deserialize_static_str_slice")]
    keywords: StaticStrSlice,
    #[serde(deserialize_with = "deserialize_static_str")]
    oneliner: StaticStr,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde(deserialize_with = "FuncModelDef::deserialize_option")]
    constructor: Option<FuncModel>,
    #[serde(deserialize_with = "FuncModelDef::deserialize_vec")]
    scope: Vec<FuncModel>,
}

#[derive(Deserialize)]
struct TypeModelWrapper(#[serde(with = "TypeModelDef")] TypeModel);

impl TypeModelDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<TypeModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|TypeModelWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<TypeModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|TypeModelWrapper(s)| s))
    }
}

#[derive(Deserialize)]
#[serde(remote = "SymbolsModel")]
pub struct SymbolsModelDef {
    name: EcoString,
    title: EcoString,
    #[serde(with = "HtmlDef")]
    details: Html,
    #[serde(deserialize_with = "SymbolModelDef::deserialize_vec")]
    list: Vec<SymbolModel>,
}

#[derive(Deserialize)]
struct SymbolsModelWrapper(#[serde(with = "SymbolsModelDef")] SymbolsModel);

impl SymbolsModelDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<SymbolsModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|SymbolsModelWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<SymbolsModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|SymbolsModelWrapper(s)| s))
    }
}

#[derive(Deserialize)]
#[serde(remote = "SymbolModel")]
#[serde(rename_all = "camelCase")]
pub struct SymbolModelDef {
    name: EcoString,
    codepoint: u32,
    accent: bool,
    unicode_name: Option<EcoString>,
    alternates: Vec<EcoString>,
    #[serde(deserialize_with = "deserialize_static_str_option")]
    markup_shorthand: Option<StaticStr>,
    #[serde(deserialize_with = "deserialize_static_str_option")]
    math_shorthand: Option<StaticStr>,
}

#[derive(Deserialize)]
struct SymbolModelWrapper(#[serde(with = "SymbolModelDef")] SymbolModel);

impl SymbolModelDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<SymbolModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|SymbolModelWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<SymbolModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|SymbolModelWrapper(s)| s))
    }
}

#[derive(Deserialize)]
#[serde(remote = "ShorthandsModel")]
pub struct ShorthandsModelDef {
    #[serde(deserialize_with = "SymbolModelDef::deserialize_vec")]
    markup: Vec<SymbolModel>,
    #[serde(deserialize_with = "SymbolModelDef::deserialize_vec")]
    math: Vec<SymbolModel>,
}

#[derive(Deserialize)]
struct ShorthandsModelWrapper(#[serde(with = "ShorthandsModelDef")] ShorthandsModel);

impl ShorthandsModelDef {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<ShorthandsModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|ShorthandsModelWrapper(s)| s).collect())
    }

    fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<ShorthandsModel>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|ShorthandsModelWrapper(s)| s))
    }
}