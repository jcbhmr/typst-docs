#!/usr/bin/env node
import { resolve } from "node:path"
import { $ } from "execa"

process.env.DEBUG ||= "execa"

const languageTags = ["es"]

const baseRoot = (process.env.BASE_URL || process.env.BASE_PATH || "/").replace(/\/?$/, "/")

for (const t of languageTags) {
    const targetDir = resolve("target")
    const base = `${baseRoot}${t}/`
    const assetsDir = resolve("public", t, "assets")
    const outFile = resolve(`pages.${t}.json`)

    await $({
        stdio: "inherit",
        cwd: resolve(t, "typst")
    })`cargo run -p typst-docs -F=cli --target-dir ${targetDir} -- --base=${base} --assets-dir=${assetsDir} --out-file=${outFile}`
}