import { DefaultTheme, LocaleConfig } from "vitepress";

export default {
  label: {{ label|json }},
  lang: {{ locale|json }},

  title: {{ title|json }},
  description: {{ description|json }},

  themeConfig: {
    sidebar: [
      {% for page1 in root_pages %}
        {
          text: {{ page1.title|json }},
          link: "/{{ locale }}{{ page1.route }}",
          {% if !page1.children.is_empty() %}
            collapsed: true,
            items: [
              {% for page2 in page1.children %}
                {% if let Some(part) = page2.part %}
                  { text: "<small>{{ part }}</small>" },
                {% endif %}
                {
                  text: {{ page2.title|json }},
                  link: "/{{ locale }}{{ page2.route }}",
                  {% if !page2.children.is_empty() %}
                    collapsed: true,
                    items: [
                      {% for page3 in page2.children %}
                        {
                          text: {{ page3.title|json }},
                          link: "/{{ locale }}{{ page3.route }}",
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