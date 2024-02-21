#!/usr/bin/env -S deno run -A
import { readFile, writeFile, mkdir, rm, cp } from "node:fs/promises";
import { resolve, join, dirname } from "node:path";
import { pathToFileURL, fileURLToPath } from "node:url";
import { $ } from "npm:execa@8.0.1";
import * as TOML from "npm:toml@3.0.0";
import process from "node:process";

process.env.NODE_DEBUG ||= "";
process.env.NODE_DEBUG += " execa";

async function build() {
  const configText = await readFile("config.toml", "utf8");
  const config = TOML.parse(configText);
  const languageTags = Object.keys(config.languages);
  const sharedCargoTargetDir = resolve("target");
  for (const t of languageTags) {
    const typstRoot = resolve(`typst-${t}`);
    const base = "/";
    const assetsDir = resolve("static");
    const { stdout } = await $({
      cwd: typstRoot,
      stdio: ["inherit", "pipe", "inherit"],
      env: {
        CARGO_TARGET_DIR: sharedCargoTargetDir,
      },
    })`cargo run -p typst-docs -F=cli -- --base=${base} --assets-dir=${assetsDir}`;
    const allPages = JSON.parse(stdout) as TypstDocs.PageModel;
    console.log(allPages);
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
    | { kind: "Html"; content: Html }
    | { kind: "Category"; content: CategoryModel }
    | { kind: "Func"; content: FuncModel }
    | { kind: "Group"; content: GroupModel }
    | { kind: "Type"; content: TypeModel }
    | { kind: "Symbols"; content: SymbolsModel }
    | { kind: "Packages"; content: Html };

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
