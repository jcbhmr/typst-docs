import { readFile } from "node:fs/promises";

export default {
  async paths() {
    const baseRoot = (
      process.env.BASE_URL ||
      process.env.BASE_PATH ||
      "/"
    ).replace(/\/?$/, "/");
    const base = `${baseRoot}es/`;
    const rootPages = JSON.parse(
      await readFile(new URL("./pages.json", import.meta.url), "utf8")
    );
    const allPages = rootPages.flatMap(function f(p) {
      return [p, ...p.children.flatMap(f)];
    });
    const params = allPages.map((page) => ({
      relativeRoute: page.route.replace(base, "") + "index",
      page,
      [page.body.kind]: page.body.content,
      base,
    }));
    console.log(params);
    return params.map((params) => ({ params }));
  },
};
