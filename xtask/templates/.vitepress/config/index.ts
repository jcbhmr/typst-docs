import { defineConfig } from "vitepress";
import root from "./root.ts";
{% for (i, locale) in locales.iter().enumerate() %}
import locale_{{ i }} from "./{{ locale.bcp47 }}.ts";
{% endfor %}

// https://vitepress.dev/reference/site-config
export default defineConfig({
  ignoreDeadLinks: true,

  srcExclude: ["**/README.md", "**/CONTRIBUTING.md"],

  base: process.env.BASE_PATH,

  locales: {
    root,
    {% for (i, locale) in locales.iter().enumerate() %}
      {{ locale.bcp47|json }}: locale_{{ i }},
    {% endfor %}
  },

  // https://vitepress.dev/reference/default-theme-config
  themeConfig: {
    logo: "/logo.png",

    socialLinks: [
      {
        icon: {
          svg: '<svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><title>Typst</title><image width="24" height="24" href="https://typst.app/assets/favicon-32x32.png" /></svg>',
        },
        link: "https://typst.app/docs",
      },
      { icon: "github", link: "https://github.com/jcbhmr/typst-docs" },
    ],

    footer: {
      message: `⚠️ This is unofficial documentation. It may be out of date.`,
      copyright: `\
Released under the <a href="https://github.com/jcbhmr/typst-docs/blob/main/LICENSE">MIT license</a>.<br />
Copyright &copy; 2024 <a href="https://typst.community/">Typst Community</a>`,
    },
  },
});