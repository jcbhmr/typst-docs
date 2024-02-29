import { resolve } from "node:path";
import { DefaultTheme, LocaleConfig, defineConfig } from "vitepress";
import es from "./es.ts";

const root = {
  lang: "en",
  label: "English",
  link: "/#",
} satisfies LocaleConfig<DefaultTheme.Config>[string];

// https://vitepress.dev/reference/site-config
export default defineConfig({
  locales: {
    root,
    es,
  },

  // https://vitejs.dev/config/
  vite: {
    resolve: {
      alias: {
        "@": resolve(".vitepress/theme"),
      },
    },
  },

  title: "My Awesome Project",
  description: "A VitePress Site",

  // https://vitepress.dev/reference/default-theme-config
  themeConfig: {
    socialLinks: [
      { icon: "github", link: "https://github.com/vuejs/vitepress" },
    ],
  },
});
