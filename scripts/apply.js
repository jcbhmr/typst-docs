#!/usr/bin/env node
import { resolve } from "node:path";
import { $ } from "execa";
import { glob } from "glob";

process.env.DEBUG ||= "execa"

for (const file of await glob("patches/**/*.patch", { nodir: true })) {
  const gitDir = file.replace(/^patches[\/\\]/, "").replace(/\.patch$/, "");
  await $({ stdio: "inherit" })`git -C ${gitDir} add -AN`;
  await $({ stdio: "inherit" })`git -C ${gitDir} reset --hard`;
  await $({ stdio: "inherit" })`git -C ${gitDir} apply --allow-empty ${resolve(file)}`;
}
