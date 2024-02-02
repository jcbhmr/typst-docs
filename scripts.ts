#!/usr/bin/env -S deno run -A
import process from "node:process";
import * as YAML from "npm:yaml";
import {
  rm,
  rename,
  access,
  writeFile,
  readFile,
  copyFile,
  chmod,
  mkdir,
  // https://github.com/denoland/deno/pull/21745#issuecomment-1924139252
  // cp,
  readdir,
} from "node:fs/promises";
import fsPromises from "node:fs/promises";
const { cp } = fsPromises;
import { resolve, join, dirname, basename } from "node:path";
import JSONParse from "npm:json-parse-even-better-errors";
import assert from "node:assert/strict";
import { $ } from "npm:execa";
import { temporaryDirectory } from "npm:tempy";
import { renderFile } from "npm:ejs";

const langs = ["zh", "es"];

async function build_1() {
  // Don't touch the '_site/' folder! That's done in step #2.
  await rm("out", { recursive: true, force: true });
  console.debug("Removed out");

  for (const lang of langs) {
    console.group(`STEP 1: Building ${lang}`);

    const langRoot = resolve(`typst-${lang}`);
    const cargoScriptFile = resolve("out", `build-${lang}.rs`);
    const outPagesFile = resolve("out", `pages-${lang}.json`);
    const tempOutDir = resolve("out", `out-${lang}`);
    console.debug(`Root for ${lang} is ${langRoot}`);
    console.debug(`Will create cargo script at ${cargoScriptFile}`);
    console.debug(`Will create pages data at ${outPagesFile}`);
    console.debug(`Output in ${tempOutDir} (processed in step 2)`);

    // prettier-ignore
    // deno-fmt-ignore
    const cargoScriptText = `\
#!/usr/bin/env -S cargo +nightly -Zscript
\`\`\`
[package]
edition = "2021"

[dependencies]
typst-docs = { path = ${JSON.stringify(join(langRoot, "crates", "typst-docs"))} }
typst = { path = ${JSON.stringify(join(langRoot, "crates", "typst"))} }
typst-render = { path = ${JSON.stringify(join(langRoot, "crates", "typst-render"))} }
serde_json = "1.0.113"
html-escape = "0.2.13"
\`\`\`

use typst_docs::{Resolver, Html, Commit, provide};
use typst::layout::Frame;
use std::env::var;
use std::fs::{File, create_dir_all};
use std::io::Write;
use typst_render::render;
use typst::visualize::Color;
use std::path::Path;

struct MyResolver;
impl Resolver for MyResolver {
  fn link(&self, _: &str) -> Option<String> {
    None
  }

  fn image(&self, filename: &str, data: &[u8]) -> String {
    eprintln!("image(self, {:?}, [{} bytes])", filename, data.len());

    format!("/assets/docs/{}", filename)
  }

  fn example(&self, hash: u128, source: Option<Html>, frames: &[Frame]) -> Html {
    eprintln!("example(self, {}, {:?}, [{} frames])", hash, source.is_some(), frames.len());

    let pixmap = render(frames.first().unwrap(), 2.0, Color::WHITE);
    let out_dir = var("OUT_DIR").expect("OUT_DIR not set");
    if let Some(source) = source {
      let filename = format!("example-{}.png", hash);
      let path = Path::new(&out_dir).join("assets/docs").join(&filename);
      create_dir_all(path.parent().unwrap()).unwrap();
      pixmap.save_png(path).unwrap();
      let src_url = format!("/assets/docs/{}", filename);

      Html::new(format!(
        r#"<div class="previewed-code">
          <pre>{}</pre>
          <div class="preview">
            <img src="{}" alt="Preview" width="480" height="190" />
          </div>
        </div>"#,
        html_escape::encode_text(source.as_str()), src_url
      ))
    } else {
      Html::new(String::new())
    }
  }

  fn commits(&self, from: &str, to: &str) -> Vec<Commit> {
    eprintln!("commits(self, {:?}, {:?})", from, to);

    vec![]
  }
}

fn main() {
  let out_pages_file = var("OUT_PAGES_FILE").expect("OUT_PAGES_FILE not set");
  let out_dir = var("OUT_DIR").expect("OUT_DIR not set");
  println!("Writing pages data to {}", out_pages_file);
  println!("Writing out files to {}", out_dir);

  println!("Running 'provide()' with MyResolver to generate pages data");
  let pages = provide(&MyResolver);
  println!("Got {} pages", pages.len());

  let json = serde_json::to_string(&pages).unwrap();
  println!("Serialized pages data to JSON ({} bytes)", json.len());

  let mut file = File::create(out_pages_file.clone()).unwrap();
  file.write_all(json.as_bytes()).unwrap();
  println!("Wrote pages data to {}", out_pages_file);
}`;
    await mkdir(dirname(cargoScriptFile), { recursive: true });
    await writeFile(cargoScriptFile, cargoScriptText);
    await chmod(cargoScriptFile, 0o755);
    console.debug(`Created ${cargoScriptFile}`);

    console.debug(`Running cargo script to generate ${outPagesFile}`);
    await $({
      stdio: "inherit",
      env: {
        OUT_PAGES_FILE: resolve(outPagesFile),
        OUT_DIR: tempOutDir,
      },
    })`${cargoScriptFile}`;
    console.debug(`Ran cargo script! See ${outPagesFile} for JSON data.`);

    console.groupEnd();
  }

  console.info(`Completed build step #1 for ${langs.length} languages`);
}

async function build_2() {
  await rm("_site", { recursive: true, force: true });
  console.debug("Removed _site");

  for (const lang of langs) {
    console.group(`Building ${lang} STEP 2`);

    const langRoot = resolve(`typst-${lang}`);
    const langOutDir = resolve("_site", lang);
    const tempOutDir = resolve("out", `out-${lang}`);
    const outPagesFile = resolve("out", `pages-${lang}.json`);
    console.debug(`Root for ${lang} is ${langRoot}`);
    console.debug(`Output in ${langOutDir}`);
    console.debug(`Using stuff from step 1 in ${tempOutDir}`);
    console.debug(`Using pages data from ${outPagesFile} step 1`);

    const langOutAssetsDocs = join(langOutDir, "assets", "docs");
    await mkdir(langOutAssetsDocs, { recursive: true });
    console.debug(`Created ${langOutAssetsDocs}`);

    const langRootAssetsFiles = join(langRoot, "assets", "files");
    for (const file of await readdir(langRootAssetsFiles)) {
      await cp(join(langRootAssetsFiles, file), join(langOutAssetsDocs, file), {
        recursive: true,
      });
      console.debug(`Copied ${file} to ${langOutAssetsDocs}`);
    }

    const tempOutAssetsDocs = join(tempOutDir, "assets", "docs");
    for (const file of await readdir(tempOutAssetsDocs)) {
      await cp(join(tempOutAssetsDocs, file), join(langOutAssetsDocs, file), {
        recursive: true,
      });
      console.debug(`Copied ${file} to ${langOutAssetsDocs}`);
    }

    await cp(
      join(langRoot, "assets", "fonts"),
      join(langOutDir, "assets", "fonts"),
      { recursive: true }
    );
    console.debug(`Copied fonts to ${langOutDir}`);

    const pages = JSONParse(await readFile(outPagesFile, "utf8"));
    console.debug(`Got pages data for ${lang}`);

    const pagesFlat = pages.flatMap(function f(page: any) {
      return [page, ...page.children.flatMap(f)];
    });
    console.info(`There are ${pagesFlat.length} pages in ${lang}`);

    for (const page of pagesFlat) {
      const htmlFile = join(
        langOutDir,
        page.route,
        page.route.endsWith("/") ? "index.html" : ""
      );

      const layoutFile = resolve("_layouts", `${page.body.kind}.ejs`);
      console.debug(`Rendering ${htmlFile} using ${layoutFile}`);
      const renderedHTML = await renderFile(
        layoutFile,
        { page },
        { async: true }
      );

      const newAssets = `${process.env.BASE_PATH || "/"}${lang}/assets/`;
      const newStyles = `${process.env.BASE_PATH || "/"}${lang}/styles/`;
      const newDocs = `${process.env.BASE_PATH || "/"}${lang}/docs/`;
      const tweakedHTML = renderedHTML
        .replaceAll(/(?<=\W)\/assets\//g, newAssets)
        .replaceAll(/(?<=\W)\/styles\//g, newStyles)
        .replaceAll(/(?<=\W)\/docs\//g, newDocs);
      console.debug(`Tweaked to '${newAssets}' and '${newDocs}'`);

      await mkdir(dirname(htmlFile), { recursive: true });
      await writeFile(htmlFile, tweakedHTML);
      console.debug(`Wrote ${htmlFile}`);
    }

    console.groupEnd();
  }

  for (const file of await readdir("public")) {
    await cp(join("public", file), join("_site", file), { recursive: true });
  }
  console.debug("Copied public to _site directly");

  console.log(`Done building ${langs.length} languages`);
}

async function build() {
  await build_1();
  await build_2();
}

async function diff() {
  for (const lang of langs) {
    console.group(`Creating patch file for ${lang}`);

    const langRoot = resolve(`typst-${lang}`);
    console.debug(`Root for ${lang} is ${langRoot}`);

    const patchFile = resolve("patches", `typst-${lang}.patch`);
    const { stdout } = await $`git -C ${langRoot} diff`;
    await mkdir(dirname(patchFile), { recursive: true });
    await writeFile(patchFile, stdout.replaceAll("\r\n", "\n"));
    console.debug(`Wrote patch to ${patchFile}`);

    console.groupEnd();
  }
}

async function apply() {
  for (const lang of langs) {
    console.group(`Applying patch file for ${lang}`);

    const langRoot = resolve(`typst-${lang}`);
    console.debug(`Root for ${lang} is ${langRoot}`);

    const patchFile = resolve("patches", `typst-${lang}.patch`);
    console.debug(`Will apply patch from ${patchFile}`);

    console.debug(`Resetting any local changes`);
    await $({ stdio: "inherit" })`git -C ${langRoot} reset --hard`;
    await $({ stdio: "inherit" })`git -C ${langRoot} clean -fd`;

    console.debug(`Attempting to apply patch`);
    await $({
      stdio: "inherit",
    })`git -C ${langRoot} apply ${patchFile} --ignore-space-change --ignore-whitespace --reject --whitespace=fix --allow-empty`;
    console.debug(`Applied patch to ${langRoot}`);

    console.groupEnd();
  }
}

async function setup() {
  console.debug("Installing the nightly Rust toolchain");
  await $({ stdio: "inherit" })`rustup toolchain add nightly`;

  console.debug("You already have Deno installed!");
  console.debug("(You're running this script with it.)");
}

const tasks = { build, diff, apply, setup, build_1, build_2 };

const taskName = process.argv[2];
assert(taskName, "No task name provided");

const task = tasks[taskName];
assert(task, `No task named '${taskName}'`);

// Disable TLA promise hanging protection
// https://github.com/denoland/deno/issues/20814
const id = setTimeout(() => {}, 10_000_000);
await task();
clearTimeout(id);
