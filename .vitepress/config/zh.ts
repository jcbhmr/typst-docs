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
            text: "<small>LANGUAGE</small>",
            items: [
              { text: "语法", link: "/zh/reference/syntax" },
              { text: "样式", link: "/zh/reference/styling" },
              { text: "脚本", link: "/zh/reference/scripting" },
            ],
          },
          {
            text: "<small>LIBRARY</small>",
            items: [
              {
                text: "基础",
                link: "https://typst.app/docs/reference/foundations/",
                items: [
                  {
                    text: "Arguments",
                    link: "https://typst.app/docs/reference/foundations/arguments/",
                  },
                  {
                    text: "Array",
                    link: "https://typst.app/docs/reference/foundations/array/",
                  },
                  {
                    text: "Assert",
                    link: "https://typst.app/docs/reference/foundations/assert/",
                  },
                  {
                    text: "Boolean",
                    link: "https://typst.app/docs/reference/foundations/boolean/",
                  },
                  {
                    text: "Bytes",
                    link: "https://typst.app/docs/reference/foundations/bytes/",
                  },
                  {
                    text: "Calculation",
                    link: "https://typst.app/docs/reference/foundations/calculation/",
                  },
                  {
                    text: "Content",
                    link: "https://typst.app/docs/reference/foundations/content/",
                  },
                  {
                    text: "Datetime",
                    link: "https://typst.app/docs/reference/foundations/datetime/",
                  },
                  {
                    text: "Dictionary",
                    link: "https://typst.app/docs/reference/foundations/dictionary/",
                  },
                  {
                    text: "Duration",
                    link: "https://typst.app/docs/reference/foundations/duration/",
                  },
                  {
                    text: "Evaluate",
                    link: "https://typst.app/docs/reference/foundations/evaluate/",
                  },
                  {
                    text: "Float",
                    link: "https://typst.app/docs/reference/foundations/float/",
                  },
                  {
                    text: "Function",
                    link: "https://typst.app/docs/reference/foundations/function/",
                  },
                  {
                    text: "Integer",
                    link: "https://typst.app/docs/reference/foundations/integer/",
                  },
                  {
                    text: "Label",
                    link: "https://typst.app/docs/reference/foundations/label/",
                  },
                  {
                    text: "Module",
                    link: "https://typst.app/docs/reference/foundations/module/",
                  },
                  {
                    text: "Panic",
                    link: "https://typst.app/docs/reference/foundations/panic/",
                  },
                  {
                    text: "Plugin",
                    link: "https://typst.app/docs/reference/foundations/plugin/",
                  },
                  {
                    text: "Regex",
                    link: "https://typst.app/docs/reference/foundations/regex/",
                  },
                  {
                    text: "Representation",
                    link: "https://typst.app/docs/reference/foundations/representation/",
                  },
                  {
                    text: "Selector",
                    link: "https://typst.app/docs/reference/foundations/selector/",
                  },
                  {
                    text: "String",
                    link: "https://typst.app/docs/reference/foundations/string/",
                  },
                  {
                    text: "Style",
                    link: "https://typst.app/docs/reference/foundations/style/",
                  },
                  {
                    text: "System",
                    link: "https://typst.app/docs/reference/foundations/system/",
                  },
                  {
                    text: "Type",
                    link: "https://typst.app/docs/reference/foundations/type/",
                  },
                  {
                    text: "Version",
                    link: "https://typst.app/docs/reference/foundations/version/",
                  },
                ],
              },
              { text: "模型", link: "/zh/reference/model/", items: [] },
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
