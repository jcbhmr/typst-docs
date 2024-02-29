import { readFile } from "node:fs/promises";

export default {
    async paths() {
        const rootPages = JSON.parse(await readFile(new URL('./pages.json', import.meta.url), "utf8"))
        const allPages = rootPages.flatMap(function f(p) {
            return [p, ...p.children.flatMap(f)]
        });
        const params = allPages.map(page => ({ route: page.path, page, [page.body.kind]: page.body.content }))
        return params.map(params => ({ params }))
    }
}