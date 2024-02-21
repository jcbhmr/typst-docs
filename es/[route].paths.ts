/// <reference lib="ESNext" />
import { readFile } from "node:fs/promises";
import type { PageModel } from "../typst-docs.ts";

export default {
  async paths() {
    let text = await readFile(
      new URL(import.meta.resolve("./typst-docs.json")),
      "utf8"
    );
    text = text.replaceAll(/(?<=[^\w\-])\/docs\//g, "/es/");
    text = text.replaceAll(/(?<=[^\w\-])\/assets\/docs\//g, "/es/assets/docs/");
    const rootPages = JSON.parse(text) as PageModel[];

    const allPages = rootPages.flatMap(function f(p): PageModel[] {
      return [p, ...p.children.flatMap(f)];
    });

    return allPages.map((page) => {
      const route = page.route.replace(/\/$/, "/index").replace(/^\/es\//, "");
      // console.log(route);
      return { params: { route, page } };
    });
  },
};
