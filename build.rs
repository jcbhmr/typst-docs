#!/usr/bin/env -S cargo +nightly -Zscript
```cargo
[package]
edition = "2021"

[dependencies]
xshell = "0.2.5"
serde = "1.0.197"
serde_json = "1.0.114"
serde_with = "3.6.1"
toml = "0.8.10"
ecow = "0.2.0"
typst-docs = { git = "https://github.com/typst/typst.git", version = "0.10.0" }
```
use xshell::{Shell, cmd};
use std::error::Error;
use serde_json::Value;
use std::{fs, env};
use crate::typst_docs_de::PageModelVecHelper;
use typst_docs::{
    provide, BodyModel, CategoryModel, Commit, FuncModel, GroupModel, Html, PageModel, Resolver,
    SymbolsModel, TypeModel,
};
use toml::Table;
use std::path::{Path, PathBuf};

// 💡 Use the "›" button in your IDE to collapse this.
mod typst_docs_de {
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
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Determine the languages. We can use the `config.toml` from Zola.
    let text = fs::read_to_string("config.toml")?;
    let config_toml = text.parse::<Table>()?;
    let languages = config_toml["languages"].as_table().ok_or("not table")?;
    let language_tags: Vec<&String> = languages.keys().collect();

    let mut base_root = env::var("BASE_URL").unwrap_or_else(|_| env::var("BASE_PATH").unwrap_or_else(|_| "/".into()));
    if !base_root.ends_with('/') {
        base_root.push('/');
    }

    // 2. Generate the `pages.json` data file for each language.
    for t in &language_tags {
        eprintln!("Generating pages data for {t}");

        let target_dir = env::current_dir()?.join("target");
        let base = format!("{base_root}{t}/");
        let assets_dir = env::current_dir()?.join("static").join(&t).join("assets");
        let out_file = env::current_dir()?.join("content").join(format!("pages.{t}.json"));

        let sh = Shell::new()?;
        sh.change_dir(t);
        cmd!(sh, "cargo run -p typst-docs -F=cli --target-dir={target_dir} -- --base={base} --assets-dir={assets_dir} --out-file={out_file}").run()?;
    }

    // 3. Use the `pages.json` data to generate Zola content pages.
    for t in &language_tags {
        eprintln!("Rendering Zola content pages for {t}");

        let out_file = env::current_dir()?.join("content").join(format!("pages.{t}.json"));
        let base = format!("{base_root}{t}/");

        let text = fs::read_to_string(out_file)?;
        let root_pages: Vec<PageModel> = serde_json::from_str(&text).map(|PageModelVecHelper(v)| v)?;
        eprintln!("{} root pages", root_pages.len());

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
        eprintln!("{} total pages", all_pages.len());

        for page in &all_pages {
            let mut path = env::current_dir()?.join("content");
            let mut route_path = page.route.to_string();
            let is_section = route_path.ends_with("/") && all_pages.iter().filter(|p| p.route != page.route).any(|p| p.route.starts_with(page.route.as_str()));
            if is_section {
                route_path.push_str(&format!("_index.{t}.md"));
            } else {
                if route_path.ends_with('/') {
                    route_path.remove(route_path.len() - 1);
                }
                route_path.push_str(&format!(".{t}.md"));
            }
            let mut route_path = route_path.replacen(&base, "/", 1);
            if route_path.starts_with('/') {
                route_path.remove(0);
            }
            path.push(route_path);

            let page_json = serde_json::to_string(&page)?;
            let kind = match &page.body {
                BodyModel::Category(_) => "category",
                BodyModel::Func(_) => "func",
                BodyModel::Group(_) => "group",
                BodyModel::Html(_) => "html",
                BodyModel::Packages(_) => "packages",
                BodyModel::Symbols(_) => "symbols",
                BodyModel::Type(_) => "type",
            };
            let template = format!("{kind}-{}.html", if is_section { "section" } else { "page" });
            let content_json = match &page.body {
                BodyModel::Category(x) => serde_json::to_string(x)?,
                BodyModel::Func(x) => serde_json::to_string(x)?,
                BodyModel::Group(x) => serde_json::to_string(x)?,
                BodyModel::Html(x) => serde_json::to_string(x)?,
                BodyModel::Packages(x) => serde_json::to_string(x)?,
                BodyModel::Symbols(x) => serde_json::to_string(x)?,
                BodyModel::Type(x) => serde_json::to_string(x)?,
            };
            let md = format!("---\ntemplate: {template}\nextra:\n  page: {page_json}\n  {kind}: {content_json}\n---");

            fs::create_dir_all(path.parent().ok_or("no parent")?)?;
            fs::write(&path, md)?;
            eprintln!("Write {path:?}");
        }
    }

    // 4. Run Zola to generate the final site!
    let base_url = base_root;
    let sh = Shell::new()?;
    cmd!(sh, "zola build -f -u={base_url}").run()?;

    Ok(())
}