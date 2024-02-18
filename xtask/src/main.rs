mod templates;
mod typst_docs_de;

use std::{
    env::{args, set_current_dir},
    error::Error,
    fs::{create_dir_all, remove_dir_all, rename, write},
    panic::catch_unwind,
    path::Path,
};

use askama::Template;
use ecow::EcoString;
use include_dir::{include_dir, Dir};
use serde::Deserialize;
use serde_json::Deserializer;
use typst_docs::{
    provide, BodyModel, CategoryModel, Commit, FuncModel, GroupModel, Html, PageModel, Resolver,
    SymbolsModel, TypeModel,
};
use xshell::{cmd, Shell};
use fancy_regex::Regex;

use crate::templates::{
    CategoryTemplate, FuncTemplate, GroupTemplate, HtmlTemplate, PackagesTemplate, SymbolsTemplate,
    TypeTemplate,
};
use crate::templates::{
    IndexTemplate, VitepressConfigIndexTemplate, VitepressConfigLocaleTemplate,
};
use crate::typst_docs_de::{PageModelDef, PageModelVecHelper};

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

struct Locale {
    name: &'static str,
    bcp47: &'static str,
    title: &'static str,
    description: &'static str,
    translations: LocaleTranslations,
}
struct LocaleTranslations {
    definitions: &'static str,
    shorthands: &'static str,
    shorthands_description: &'static str,
    within_markup_mode: &'static str,
    within_math_mode: &'static str,
}

const LOCALES: &[Locale] = &[
    Locale {
        name: "español",
        bcp47: "es",
        title: "Documentación de Typst",
        description: "Aprenda a utilizar Typst para redactar documentos más rápido. Comience con el tutorial o profundizar en la referencia.",
        translations: LocaleTranslations {
            definitions: "Definiciones",
            shorthands: "Taquigrafías",
            shorthands_description: r#"
<p>
  Las taquigrafías son secuencias concisas de caracteres que evocan glifos
  específicos. Las taquigrafías y otras formas de producir símbolos se pueden
  utilizar indistintamente. Puede utilizar diferentes conjuntos de taquigrafías
  en el modo matemático y de marcado. Algunas abreviaturas, como
  <code>~</code> para un espacio que no se separa, producen símbolos que no se
  imprimen, que se indican con un texto de marcador de posición gris.
</p>
<p>
  Puedes desactivar la interpretación de una taquigrafía escapando de cualquiera
  de sus caracteres. Si aplica el carácter de escape a un solo carácter en una
  taquigrafía, los caracteres restantes sin escape pueden formar una taquigrafía
  diferente.
</p>
"#,
            within_markup_mode: "Dentro del modo de marcado",
            within_math_mode: "Dentro del modo matemático",
        },
    },
    Locale {
        name: "中文",
        bcp47: "zh",
        title: "Typst 文档",
        description: "学习如何使用 Typst 更快地撰写文档。从教程开始，或者深入参考。",
        translations: LocaleTranslations {
            definitions: "定义",
            shorthands: "速记法",
            shorthands_description: r#"
<p>
  简写是能够唤起特定字形的简洁字符序列。
  速记法和其他产生符号的方法可以互换使用。
  您可以在数学和标记模式下使用不同的速记集。 某些简写（例如用于不间断空格的
  <code>~</code>）会生成非打印符号，这些符号用灰色占位符文本表示。
</p>
<p>
  您可以通过转义任何字符来停用速记的解释。
  如果对简写中的单个字符进行转义，则其余未转义的字符可能会形成不同的简写。
</p>
"#,
            within_markup_mode: "在标记模式下",
            within_math_mode: "在数学模式下",
        },
    },
];

fn build() -> Result<(), Box<dyn Error>> {
    let sh = Shell::new()?;
    remove_dir_all("out")?;
    for locale in LOCALES {
        let bcp47 = &locale.bcp47;
        eprintln!("Building {bcp47} docs");

        let manifest_path = format!("crates/typst-{bcp47}/crates/typst-docs/Cargo.toml");
        let out_dir = Path::new("out").join(&bcp47);
        let public_dir = Path::new("out").join("public").join(&bcp47);

        let stdout = cmd!(
            sh,
            "cargo run --manifest-path {manifest_path} --features=cli --target-dir=target -- --out-dir {public_dir}"
        )
        .read()?;
        print!("{stdout}");

        let root_pages: Vec<PageModel> =
            serde_json::from_str(&stdout).map(|PageModelVecHelper(v)| v)?;
        eprintln!("Parsed data for {} root pages", root_pages.len());

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
        eprintln!(
            "Crawled root pages and found data for {} total pages",
            all_pages.len()
        );

        for page in &all_pages {
            let mut path = out_dir.clone();
            let mut route_path = page.route.to_string();
            if route_path.ends_with("/") {
                route_path.push_str("index.md");
            }
            if route_path.starts_with("/") {
                route_path.remove(0);
            }
            path.push(route_path);

            let html = match &page.body {
                BodyModel::Category(category) => CategoryTemplate {
                    locale,
                    page,
                    category,
                }
                .render()?,
                BodyModel::Func(func) => FuncTemplate { locale, page, func }.render()?,
                BodyModel::Group(group) => GroupTemplate {
                    locale,
                    page,
                    group,
                }
                .render()?,
                BodyModel::Html(html) => HtmlTemplate { locale, page, html }.render()?,
                BodyModel::Packages(packages) => PackagesTemplate {
                    locale,
                    page,
                    packages,
                }
                .render()?,
                BodyModel::Symbols(symbols) => SymbolsTemplate {
                    locale,
                    page,
                    symbols,
                }
                .render()?,
                BodyModel::Type(type_) => TypeTemplate {
                    locale,
                    page,
                    type_,
                }
                .render()?,
            };
            let re = Regex::new(r#"(?<=[^\w\-])/docs/"#).unwrap();
            let html = re.replace_all(&html, format!("/{bcp47}/docs/").as_str());
            let re = Regex::new(r#"(?<=[^\w\-])/assets/docs/"#).unwrap();
            let html = re.replace_all(&html, format!("/{bcp47}/assets/docs/").as_str());
            eprintln!(
                "Generated {} chars of HTML for {:?}",
                html.len(),
                &page.route
            );

            create_dir_all(path.parent().ok_or("no parent")?)?;
            write(&path, html.into_owned())?;
            eprintln!("Created {:?}", &path);
        }
        eprintln!("Created all Markdown files for {bcp47}");

        let ts = VitepressConfigLocaleTemplate {
            locale,
            root_pages: &root_pages,
        }
        .render()?;
        let path = Path::new("out")
            .join(".vitepress")
            .join("config")
            .join(format!("{bcp47}.ts"));
        create_dir_all(path.parent().ok_or("no parent")?)?;
        write(&path, ts)?;
        eprintln!("Created {bcp47} config file {path:?}");
    }

    let ts = VitepressConfigIndexTemplate { locales: LOCALES }.render()?;
    let path = Path::new("out")
        .join(".vitepress")
        .join("config")
        .join("index.ts");
    create_dir_all(path.parent().ok_or("no parent")?)?;
    write(&path, ts)?;
    eprintln!("Created index config file {path:?}");

    let index = IndexTemplate { locales: LOCALES }.render()?;
    let path = Path::new("out").join("index.md");
    create_dir_all(path.parent().ok_or("no parent")?)?;
    write(&path, index)?;
    eprintln!("Created index file {path:?}");

    create_dir_all("out")?;
    ASSETS.extract("out")?;
    eprintln!("Extracted static assets to out");

    let sh = Shell::new()?;
    create_dir_all("out")?;
    sh.change_dir("out");
    cmd!(sh, "npm install").run()?;
    cmd!(sh, "npm run build").run()?;

    remove_dir_all("_site")?;
    rename("out/.vitepress/dist", "_site")?;
    eprintln!("Moved final site to '_site'");

    Ok(())
}

fn diff() -> Result<(), Box<dyn Error>> {
    let sh = Shell::new()?;
    create_dir_all("patches")?;
    for locale in LOCALES {
        let bcp47 = &locale.bcp47;
        let patch_file = format!("patches/typst-{bcp47}.patch");
        cmd!(sh, "git -C crates/typst-{bcp47} reset").run()?;
        cmd!(sh, "git -C crates/typst-{bcp47} add -AN").run()?;
        let stdout = cmd!(sh, "git -C crates/typst-{bcp47} diff --binary").read()?;
        write(&patch_file, stdout)?;
        eprintln!("Collected stdout and wrote to {patch_file:?}");
    }
    Ok(())
}

fn apply() -> Result<(), Box<dyn Error>> {
    let sh = Shell::new()?;
    for locale in LOCALES {
        let bcp47 = &locale.bcp47;
        let patch_file = format!("patches/typst-{bcp47}.patch");
        cmd!(sh, "git -C crates/typst-{bcp47} add -AN").run()?;
        cmd!(sh, "git -C crates/typst-{bcp47} reset --hard").run()?;
        cmd!(sh, "git -C crates/typst-{bcp47} apply ../../{patch_file}").run()?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    match args().nth(1).ok_or("no task specified")?.as_str() {
        "build" => build(),
        "diff" => diff(),
        "apply" => apply(),
        _ => Err("task not found".into()),
    }
}
