import { DefaultTheme, LocaleConfig } from "vitepress";

export default {
  label: {{ locale.name|json }},
  lang: {{ locale.bcp47|json }},

  link: "/{{ locale.bcp47 }}/docs/",

  title: {{ locale.title|json }},
  description: {{ locale.description|json }},

  themeConfig: {
    sidebar: [
      {% for page1 in root_pages %}
        {
          text: {{ page1.title|json }},
          link: "/{{ locale.bcp47 }}{{ page1.route }}",
          {% if !page1.children.is_empty() %}
            collapsed: true,
            items: [
              {% for page2 in page1.children %}
                {% if let Some(part) = page2.part %}
                  { text: "<small>{{ part }}</small>" },
                {% endif %}
                {
                  text: {{ page2.title|json }},
                  link: "/{{ locale.bcp47 }}{{ page2.route }}",
                  {% if !page2.children.is_empty() %}
                    collapsed: true,
                    items: [
                      {% for page3 in page2.children %}
                        {
                          text: {{ page3.title|json }},
                          link: "/{{ locale.bcp47 }}{{ page3.route }}",
                        },
                      {% endfor %}
                    ],
                  {% endif %}
                },
              {% endfor %}
            ],
          {% endif %}
        },
      {% endfor %}
    ],
  },
} satisfies LocaleConfig<DefaultTheme.Config>[string];