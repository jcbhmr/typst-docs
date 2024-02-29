#!/usr/bin/env node
import { mkdir, writeFile } from "node:fs/promises";
import { $ } from "execa";
import { dirname } from "node:path";

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
