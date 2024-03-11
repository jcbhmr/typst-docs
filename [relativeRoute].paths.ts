import { readFile } from "node:fs/promises";

export default {
  async paths() {
    const languageTags = ["es"];

    const baseRoot = (
      process.env.BASE_URL ||
      process.env.BASE_PATH ||
      "/"
    ).replace(/\/?$/, "/");

    const base: Record<string, string> = { __proto__: null! };
    for (const t of languageTags) {
      base[t] = `${baseRoot}${t}/`;
    }

    const rootPages: Record<string, any> = { __proto__: null! };
    for (const t of languageTags) {
      rootPages[t] = JSON.parse(
        await readFile(new URL(`./pages.${t}.json`, import.meta.url), "utf8"),
      );
    }

    const allPages: Record<string, any[]> = { __proto__: null! };
    for (const t of languageTags) {
      allPages[t] = rootPages[t].flatMap(function f(p: any) {
        return [p, ...p.children.flatMap(f)];
      });
    }

    const params: any[] = [];
    for (const t of languageTags) {
      params.push(
        ...allPages[t].map(({ children, ...page }: any) => ({
          relativeRoute: page.route.replace("/", "") + "index",
          page,
          base: base[t],
        })),
      );
    }
    return params.map((params) => ({ params }));
  },
};
