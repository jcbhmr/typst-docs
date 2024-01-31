import { DefaultTheme, LocaleConfig } from "vitepress";

export default {
  label: "English",
  lang: "en",

  // Normally tries to take you to that page in the target language.
  // This way it just takes you to the index page regardless
  // of what language page you're coming from.
  link: "/#",

  title: "Typst Documentation",
  description: "📗 Typst documentation in more languages",

  themeConfig: {},
} satisfies LocaleConfig<DefaultTheme.Config>["root"];
