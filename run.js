#!/usr/bin/env node
async function apply() {
    const { resolve } = await import ("node:path");
    const { $ } = await import ("execa");
    const { glob } = await import ("glob");

    process.env.DEBUG ||= "execa"

    for (const file of await glob("patches/**/*.patch", { nodir: true })) {
    const gitDir = file.replace(/^patches[\/\\]/, "").replace(/\.patch$/, "");
    await $({ stdio: "inherit" })`git -C ${gitDir} add -AN`;
    await $({ stdio: "inherit" })`git -C ${gitDir} reset --hard`;
    await $({ stdio: "inherit" })`git -C ${gitDir} apply --allow-empty ${resolve(file)}`;
    }
}

async function diff() {
    const { mkdir, writeFile } = await import ("node:fs/promises");
    const { $ } = await import ("execa");
    const { dirname } = await import ("node:path");

    process.env.DEBUG ||= "execa"

    const { stdout } = await $`git config --file .gitmodules --get-regexp path`;
    const submodulePaths = stdout
    .split(/\r?\n/g)
    .map((line) => line.slice(line.indexOf(" ") + 1));
    for (const path of submodulePaths) {
    const patchFile = `patches/${path}.patch`;
    await mkdir(dirname(patchFile), { recursive: true });
    await $({ stdio: "inherit" })`git -C ${path} reset`;
    await $({ stdio: "inherit" })`git -C ${path} add -AN`;
    const { stdout } = await $({
        stdio: ["inherit", "pipe", "inherit"],
    })`git -C ${path} diff --binary`;
    await writeFile(patchFile, stdout);
    }

}

async function generate() {
    const { resolve } = await import ("node:path")
    const { $ } = await import ("execa")

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
}

await { apply }[process.argv[2]]()