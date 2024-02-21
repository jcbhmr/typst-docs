import { DefaultTheme, LocaleConfig } from "vitepress";
import { readFile } from "node:fs/promises";
import { PageModel } from "../../typst-docs.ts";

let text = await readFile(
  new URL(import.meta.resolve("../../es/typst-docs.json")),
  "utf8"
);
text = text.replaceAll(/(?<=[^\w\-])\/docs\//g, "/es/");
text = text.replaceAll(/(?<=[^\w\-])\/assets\/docs\//g, "/assets/es/");
const rootPages = JSON.parse(text) as PageModel[];

export default {
  label: "español",
  lang: "es",
  title: "",
  description: "",
  themeConfig: {
    sidebar: rootPages.map(function f(p): DefaultTheme.SidebarItem {
      const item: DefaultTheme.SidebarItem = { text: p.title, link: p.route };
      if (p.children.length) {
        item.collapsed = true;
        item.items = p.children.map(f);
      }
      return item;
    }),
  },
} satisfies LocaleConfig<DefaultTheme.Config>[string];
