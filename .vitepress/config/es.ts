import { readFile } from "node:fs/promises";
import { DefaultTheme, LocaleConfig } from "vitepress";

const baseRoot = (process.env.BASE_URL || process.env.BASE_PATH || "/").replace(
  /\/?$/,
  "/"
);
const base = `${baseRoot}es/`;
const rootPages = JSON.parse(await readFile("es/pages.json", "utf8"));
const sidebar = rootPages.map(function f(page) {
  const item: any = {
    text: page.title,
    link: page.route.replace(base, "/es/") + "index",
  };
  if (page.children.length) {
    item.collapsed = true;
    item.items = page.children.map(f);
  }
  return item;
});

export default {
  lang: "es",
  label: "español",

  themeConfig: {
    sidebar,
  },
} satisfies LocaleConfig<DefaultTheme.Config>[string];
