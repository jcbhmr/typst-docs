import { DefaultTheme, LocaleConfig } from "vitepress";

export default {
  label: {{ label|json }},
  lang: {{ locale|json }},

  title: {{ title|json }},
  description: {{ description|json }},

  themeConfig: {
    sidebar: [
      
    ],
  },
} satisfies LocaleConfig<DefaultTheme.Config>[string];