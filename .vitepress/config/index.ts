import { defineConfig } from "vitepress";
import root from "./root.ts";
import fr from "./fr.ts";
import zh from "./zh.ts";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  srcExclude: ["**/README.md", "**/CONTRIBUTING.md"],

  locales: { root, fr, zh },

  // https://vitepress.dev/reference/default-theme-config
  themeConfig: {
    socialLinks: [
      { icon: "github", link: "https://typst.app/docs" },
      { icon: "github", link: "https://github.com/vuejs/vitepress" },
    ],

    footer: {
      message: `⚠️ This is unofficial documentation. It may be out of date.`,
      copyright: `\
Released under the <a href="https://github.com/jcbhmr/typst-docs/blob/main/LICENSE">MIT license</a>.<br />
Copyright &copy; 2024 <a href="https://typst.community/">Typst Community</a>`,
    },
  },
});
