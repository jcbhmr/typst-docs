import { LocaleConfig, defineConfig } from "vitepress";
import es from "./es.ts";

const root = {
  label: "English",
  lang: "en",
  title: "My Awesome Project",
  description: "A VitePress Site",
} satisfies LocaleConfig[string];

// https://vitepress.dev/reference/site-config
export default defineConfig({
  srcExclude: ["**/README.md", "**/CONTRIBUTING.md", "*/typst"],

  locales: { root, es },

  title: "My Awesome Project",
  description: "A VitePress Site",

  // https://vitepress.dev/reference/default-theme-config
  themeConfig: {
    socialLinks: [
      { icon: "github", link: "https://github.com/vuejs/vitepress" },
    ],
  },
});
