import { DefaultTheme, LocaleConfig } from "vitepress";

export default {
  label: "Français",
  lang: "fr",

  title: "Documentation Typst",
  description: "",

  themeConfig: {
    sidebar: [
      { text: "Aperçu", link: "/fr/" },
      {
        text: "Didacticiel",
        link: "/fr/tutorial/",
        collapsed: true,
        items: [
          { text: "Écrire en Typst", link: "/fr/tutorial/writing-in-typst" },
          {
            text: "Mise en page",
            link: "https://typst.app/docs/tutorial/formatting",
          },
          {
            text: "Style avancé",
            link: "https://typst.app/docs/tutorial/advanced-styling",
          },
          {
            text: "Créer un modèle",
            link: "https://typst.app/docs/tutorial/making-a-template",
          },
        ],
      },
      {
        text: "Référence",
        link: "https://typst.app/docs/reference/",
        collapsed: true,
        items: [
          {
            text: "<small>LANGUE</small>",
            items: [
              { text: "Syntaxe", link: "https://typst.app/docs/reference/" },
              { text: "Coiffant", link: "https://typst.app/docs/reference/" },
              { text: "Script", link: "https://typst.app/docs/reference/" },
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
} satisfies LocaleConfig<DefaultTheme.Config>["fr"];
