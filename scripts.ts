#!/usr/bin/env -S deno run -A
import { readFile, writeFile, mkdir, rm, cp } from "node:fs/promises";
import { existsSync } from "node:fs"
import { resolve, join, dirname } from "node:path";
import { pathToFileURL, fileURLToPath } from "node:url";
import { $ } from "npm:execa@8.0.1";
import * as TOML from "npm:toml@3.0.0";
import process from "node:process";

process.env.NODE_DEBUG ||= "";
process.env.NODE_DEBUG += " execa";

const configText = await readFile("config.toml", "utf8");
const config = TOML.parse(configText) as ZolaConfig.Config;
const languageTags = Object.keys(config.languages);

const sharedCargoTargetDir = resolve("target");
const assetsDir = resolve("static/assets");
const contentDir = resolve("content");

async function build() {
  for (const t of languageTags) {
    const { stdout } = await $({
      cwd: resolve(`typst-${t}`),
      stdio: ["inherit", "pipe", "inherit"],
      env: {
        CARGO_TARGET_DIR: sharedCargoTargetDir,
      },
    })`cargo run -p typst-docs -F=cli -- --base=${`/${t}/`} --assets-dir=${assetsDir}`;
    console.log(stdout)
    const rootPages = JSON.parse(stdout) as TypstDocs.PageModel;
    const allPages = rootPages.flatMap(function f(p): TypstDocs.PageModel[] {
      return [p, ...p.children.flatMap(f)];
    });
    for (const p of allPages) {
      if (p.body.kind !== "html") continue;
      let routePath = p.route; // "/es/community/"
      routePath = "." + routePath.slice(`/${t}`.length); // "./community/"
      routePath = routePath.slice(0, -1) + `.${t}.md` // "./community.es.md"
      const md = `\
---
title: ${JSON.stringify(p.title)}
template: ${p.body.kind}.html
extra:
  page: ${JSON.stringify(p)}
  ${p.body.kind}: ${JSON.stringify(p.body.content)}
---`;
      const path = resolve(contentDir, routePath);
      await mkdir(dirname(path), { recursive: true });
      await writeFile(path, md);
      console.error("%cWrite%c %o", "color:green;font-weight:bold", "", path);
    }
  }
}

const tasks: Record<string, () => Promise<any>> = {
  __proto__: null!,
  build,
};
const taskName = process.argv[2] || throw_();
const task = tasks[taskName] ?? throw_();
await task();

namespace TypstDocs {
  export type Html = string;

  export interface PageModel {
    route: string;
    title: string;
    description: string;
    part: string | null;
    outline: OutlineItem[];
    body: BodyModel;
    children: PageModel[];
  }

  export interface OutlineItem {
    id: string;
    name: string;
    children: OutlineItem[];
  }

  export type BodyModel =
    | { kind: "html"; content: Html }
    | { kind: "category"; content: CategoryModel }
    | { kind: "func"; content: FuncModel }
    | { kind: "group"; content: GroupModel }
    | { kind: "type"; content: TypeModel }
    | { kind: "symbols"; content: SymbolsModel }
    | { kind: "packages"; content: Html };

  export interface CategoryModel {
    name: string;
    title: string;
    details: Html;
    items: CategoryItem[];
    shorthands: ShorthandsModel | null;
  }

  export interface CategoryItem {
    name: string;
    route: string;
    oneliner: string;
    code: boolean;
  }

  export interface FuncModel {
    path: string[];
    name: string;
    title: string;
    keywords: string[];
    oneliner: string;
    element: boolean;
    details: Html;
    example: Html | null;
    self_: boolean;
    params: ParamModel[];
    returns: string[];
    scope: FuncModel[];
  }

  export interface ParamModel {
    name: string;
    details: Html;
    example: Html | null;
    types: string[];
    strings: StrParam[];
    default: Html | null;
    positional: boolean;
    named: boolean;
    required: boolean;
    variadic: boolean;
    settable: boolean;
  }

  export interface StrParam {
    string: string;
    details: Html;
  }

  export interface GroupModel {
    name: string;
    title: string;
    details: Html;
    functions: FuncModel[];
  }

  export interface TypeModel {
    name: string;
    title: string;
    keywords: string[];
    oneliner: string;
    details: Html;
    constructor: FuncModel | null;
    scope: FuncModel[];
  }

  export interface SymbolsModel {
    name: string;
    title: string;
    details: Html;
    list: SymbolModel[];
  }

  export interface SymbolModel {
    name: string;
    codepoint: number;
    accent: boolean;
    unicodeName: string | null;
    alternates: string[];
    markupShorthand: string | null;
    mathShorthand: string | null;
  }

  export interface ShorthandsModel {
    markup: SymbolModel[];
    math: SymbolModel[];
  }
}

namespace ZolaConfig {
  export interface Config {
    languages: Record<string, LanguageConfig>;
  }

  export interface LanguageConfig {}
}
