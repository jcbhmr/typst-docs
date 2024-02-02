#!/usr/bin/env -S deno run -A
import process from "node:process";
import * as YAML from "npm:yaml";
import { readFile, copyFile } from "node:fs/promises";
import { resolve } from "node:path";
import JSONParse from "npm:json-parse-even-better-errors";

function throw_(error: any): never {
  throw error;
}

const langs = ["zh", "es"];

async function build() {
  for (const lang of langs) {
    const langRoot = resolve(lang);
    const langData = JSONParse(
      await readFile(resolve(langRoot, "data.json"), "utf8")
    );
  }
  await copyFile("index.html", "_site/index.html");
}

const tasks = { build };
const taskName =
  process.argv[2] || throw_(new ReferenceError("No task provided"));
const task =
  tasks[taskName] ?? throw_(new ReferenceError(`No such task '${taskName}'`));
await task();
