<template>
  <h1>{{ $props.page.title }}</h1>
  <div v-html="$props.packages"></div>
  <p><b style="font-size: 1.2em">👉 <a href="https://typst.app/docs/packages/">typst.app/docs/packages</a></b></p>
</template>

<script setup lang="ts">
defineProps<{
  page: object;
  packages: string;
}>();
</script>
