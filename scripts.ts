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
} from "node:fs/promises";
import { resolve, join, dirname, basename } from "node:path";
import JSONParse from "npm:json-parse-even-better-errors";
import assert from "node:assert/strict";
import { $ } from "npm:execa";
import { temporaryDirectory } from "npm:tempy";

const langs = ["zh", "es"];

async function build() {
  await rm("_site", { recursive: true, force: true });
  console.debug("Removed _site");
  await rm("out", { recursive: true, force: true });
  console.debug("Removed out");

  for (const lang of langs) {
    console.group(`Building ${lang}`);

    const langRoot = resolve(`typst-${lang}`);
    console.debug(`Root for ${lang} is ${langRoot}`);
    const cargoScriptFile = resolve("out", `build-${lang}.rs`);
    console.debug(`Will create cargo script at ${cargoScriptFile}`);
    const outPagesFile = resolve("out", `pages-${lang}.json`);
    console.debug(`Will create pages data at ${outPagesFile}`);

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
dataurl = "0.1.2"
mime_guess = "2.0.4"
html-escape = "0.2.13"
\`\`\`

use typst_docs::{Resolver, Html, Commit, provide};
use typst::layout::Frame;
use std::env::args;
use std::fs::File;
use std::io::Write;
use dataurl::DataUrl;
use mime_guess;
use typst_render::render;
use typst::visualize::Color;

struct MyResolver;
impl Resolver for MyResolver {
  fn link(&self, _: &str) -> Option<String> {
    None
  }

  fn image(&self, filename: &str, data: &[u8]) -> String {
    eprintln!("image(self, {:?}, [{} bytes])", filename, data.len());

    let mut data_url = DataUrl::new();
    data_url.set_data(data);

    let mime_type = mime_guess::from_path(filename).first_or_octet_stream();
    data_url.set_media_type(Some(mime_type.as_ref().into()));

    data_url.to_string()
  }

  fn example(&self, hash: u128, source: Option<Html>, frames: &[Frame]) -> Html {
    eprintln!("example(self, {}, {:?}, [{} frames])", hash, source.is_some(), frames.len());

    let pixmap = render(frames.first().unwrap(), 2.0, Color::WHITE);
    if let Some(source) = source {
      let mut data_url = DataUrl::new();
      data_url.set_data(pixmap.data());
      data_url.set_media_type(Some("image/png".into()));

      Html::new(format!(
        r#"<div class="previewed-code">
          <pre>{}</pre>
          <div class="preview">
            <img src="{}" alt="Preview" width="480" height="190" />
          </div>
        </div>"#,
        html_escape::encode_text(source.as_str()), data_url.to_string()
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
  let out_pages_file = args().nth(1).unwrap();
  println!("Writing pages data to {}", out_pages_file);

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
    await $({ stdio: "inherit" })`${cargoScriptFile} ${outPagesFile}`;

    const pagesData = JSONParse(await readFile(outPagesFile, "utf8"));
    console.debug(`Got pages data for ${lang}`);

    console.groupEnd();
  }

  await mkdir("_site", { recursive: true });
  await copyFile("index.html", "_site/index.html");
  console.debug("Copied index.html to _site");

  console.log(`Done building ${langs.length} languages`);
}

async function diff() {
  for (const lang of langs) {
    console.group(`Creating patch file for ${lang}`);

    const langRoot = resolve(`typst-${lang}`);
    console.debug(`Root for ${lang} is ${langRoot}`);

    const patchText = await (async () => {
      const { stdout } = await $`git -C ${langRoot} diff`;
      return stdout;
    })();
    console.debug(`Got diff for ${lang} (${patchText.length} bytes)`);

    const patchFile = resolve("patches", `typst-${lang}.patch`);
    await mkdir(dirname(patchFile), { recursive: true });
    await writeFile(patchFile, patchText);
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
    await $`git -C ${langRoot} apply ${patchFile}`;
    console.debug(`Applied patch to ${langRoot}`);

    console.groupEnd();
  }
}

async function setup() {
  console.debug("Installing the nightly Rust toolchain");
  await $({ stdio: "inherit" })`rustup toolchain add nightly`;
}

const tasks = { build, diff, apply, setup };

const taskName = process.argv[2];
assert(taskName, "No task name provided");

const task = tasks[taskName];
assert(task, `No task named '${taskName}'`);

// Disable TLA promise hanging protection
// https://github.com/denoland/deno/issues/20814
const id = setTimeout(() => {}, 10_000_000);
await task();
clearTimeout(id);
