<script setup>
const packages = {{ packages.as_str()|json }}
</script>

<div v-html="packages"></div>

👉 [typst.app/docs/packages](https://typst.app/docs/packages/)
