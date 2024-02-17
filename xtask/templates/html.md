<script setup>
const html = {{ html.as_str()|json }}
</script>

<div v-html="html"></div>