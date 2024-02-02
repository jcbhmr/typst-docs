#!/usr/bin/env -S deno run -A
import process from "node:process";
import * as YAML from "npm:yaml";
import { readFile, copyFile } from "node:fs/promises";
import { resolve } from "node:path";
import JSONParse from "npm:json-parse-even-better-errors";
import { rm, rename, access } from "node:fs/promises";
import assert from "node:assert/strict";
import { $ } from "npm:execa";
import { temporaryDirectory } from "npm:tempy";

const langs = ["zh", "es"];

const getCargoScriptText = ({
  typstDocsDir,
  outPagesFile,
}: {
  typstDocsDir: string;
  outPagesFile: string;
}) => `\
#!/usr/bin/env -S cargo +nightly -Zscript
\`\`\`
[package]
edition = "2021"

[dependencies]
typst-docs = { path = ${JSON.stringify(typstDocsDir)} }
\`\`\`

fn build() {
  println!("Building...");

}

fn main() {
  match std::env::args().nth(1) {
    Some("build") => build(),
    Some(_) => panic!("No script named {:?}", std::env::args().nth(1)),
    None => panic!("No script name provided"),
  }
}`;

async function build() {
  await rm("_site", { recursive: true, force: true });
  console.debug("Removed _site");
  await rm("out", { recursive: true, force: true });
  console.debug("Removed out");

  for (const lang of langs) {
    console.group(`Building ${lang}`);

    const langRoot = resolve(`typst-${lang}`);
    const tempDir = temporaryDirectory();
    const outPagesFile = resolve("out", `typst-${lang}-pages.json`);

    const langData = JSONParse(await readFile(outPagesFile, "utf8"));
    console.debug(`Read data for ${lang}`);

    console.groupEnd();
  }

  await copyFile("index.html", "_site/index.html");
  console.debug("Copied index.html to _site");

  console.log(`Done building ${langs.length} languages`);
}

async function version() {
  const newVersion = process.argv[3];
  assert(newVersion.match(/^\d+\.\d+\.\d+$/), "Invalid version number");
  console.debug(`New version: ${newVersion}`);

  const newTag = `v${newVersion}`;
  console.debug(`New tag: ${newTag}`);

  for (const lang of langs) {
    const langRoot = resolve(`typst-${lang}`);

    const oldVersion = await (async () => {
      const { stdout } = await $`git -C ${langRoot} describe --tags`;
      assert(stdout, `No stdout from 'git describe' in ${langRoot}`);
      const match = stdout.match(/^v(\d+\.\d+\.\d+)$/);
      assert(match, `'$stdout' does not match vN.N.N`);
      return match[1];
    })();
    console.debug(`Old version: ${oldVersion}`);

    await $`git -C ${langRoot} reset --hard`;
    await $`git -C ${langRoot} checkout ${newTag}`;
    console.info(`Checked out ${newTag} in ${langRoot}`);

    const oldPatchFile = resolve(
      "oatches",
      `typst-${lang}+${oldVersion}.patch`
    );
    const newPatchFile = resolve(
      "oatches",
      `typst-${lang}+${newVersion}.patch`
    );
    console.debug(`Old patch file: ${oldPatchFile}`);
    console.debug(`New patch file: ${newPatchFile}`);
    assert(
      await access(oldPatchFile).then(
        () => true,
        () => false
      ),
      `Patch file ${oldPatchFile} does not exist`
    );
    assert(
      await access(newPatchFile).then(
        () => false,
        () => true
      ),
      `Patch file ${newPatchFile} already exists`
    );

    console.info(`Attempting to migrate ${oldPatchFile} to ${newPatchFile}`);
    if (
      await $`git -C ${langRoot} apply ../${oldPatchFile}`.then(
        () => true,
        () => false
      )
    ) {
      await rename(oldPatchFile, newPatchFile);
      console.info(`Migrated ${oldPatchFile} to ${newPatchFile}`);
    } else {
      console.warn(`Failed to migrate ${oldPatchFile} to ${newPatchFile}`);
    }
  }
}

const tasks = { build };

const taskName = process.argv[2];
assert(taskName, "No task name provided");

const task = tasks[taskName];
assert(task, `No task named '${taskName}'`);

await task();
