---
# https://vitepress.dev/reference/default-theme-home-page
layout: home
pageClass: my-index-page

features:
  - title: English
    link: https://typst.app/docs
{% for locale in locales %}
  - title: {{ locale.name }}
    link: /{{ locale.bcp47 }}/docs/
{% endfor %}
---

<style>
  .my-index-page .VPContent {
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .my-index-page .VPContent .VPLink .title {
    font-size: 1.5em;
    line-height: 1.46;
    width: 100px;
    text-align: center;
    white-space: nowrap;
  }
</style>

<div align=center>
<br />

[✍ Add your own translations!](https://github.com/typst-community/typst-docs/blob/main/CONTRIBUTING.md)

</div>