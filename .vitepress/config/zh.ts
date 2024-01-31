import { DefaultTheme, LocaleConfig } from "vitepress";

export default {
  label: "简体中文",
  lang: "zh",

  title: "打字员文档",
  description: "",

  themeConfig: {
    sidebar: [
      { text: "概览", link: "/zh/" },
      {
        text: "教程",
        link: "/zh/tutorial/",
        collapsed: true,
        items: [
          { text: "使用 Typst 写作", link: "/zh/tutorial/writing-in-typst" },
          {
            text: "格式",
            link: "/zh/tutorial/formatting",
          },
          {
            text: "高级样式",
            link: "/zh/tutorial/advanced-styling",
          },
          {
            text: "制作模板",
            link: "/zh/tutorial/making-a-template",
          },
        ],
      },
      {
        text: "参考",
        link: "/zh/reference/",
        collapsed: true,
        items: [
          {
            text: "<small>LANGUE</small>",
            items: [
              { text: "语法", link: "/zh/reference/syntax" },
              { text: "样式", link: "/zh/reference/styling" },
              { text: "脚本", link: "/zh/reference/scripting" },
            ],
          },
          {
            text: "<small>BIBLIOTHÈQUE</small>",
            items: [
              { text: "Fondations", link: "https://typst.app/docs/reference/" },
              { text: "Modèle", link: "https://typst.app/docs/reference/" },
              { text: "Texte", link: "https://typst.app/docs/reference/" },
              {
                text: "Mathématiques",
                link: "https://typst.app/docs/reference/",
              },
              { text: "Symboles", link: "https://typst.app/docs/reference/" },
              {
                text: "Mise en page",
                link: "https://typst.app/docs/reference/",
              },
              { text: "Visualiser", link: "https://typst.app/docs/reference/" },
              {
                text: "Introspection",
                link: "https://typst.app/docs/reference/",
              },
              {
                text: "Chargement des données",
                link: "https://typst.app/docs/reference/",
              },
            ],
          },
        ],
      },
      {
        text: "Guides",
        link: "https://typst.app/docs/guides/",
        collapsed: true,
        items: [
          {
            text: "Guide pour les utilisateurs de LaTeX",
            link: "https://typst.app/docs/guides/",
          },
          {
            text: "Guide de mise en page",
            link: "https://typst.app/docs/guides/",
          },
        ],
      },
      { text: "Paquets", link: "https://typst.app/docs/packages/" },
      {
        text: "Journal des modifications",
        link: "https://typst.app/docs/changelog/",
      },
      { text: "Feuille de route", link: "https://typst.app/docs/roadmap/" },
      { text: "Communauté", link: "https://typst.app/docs/community/" },
    ],
  },
} satisfies LocaleConfig<DefaultTheme.Config>["zh"];
