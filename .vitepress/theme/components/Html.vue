<template>
  <div v-html="$props.html"></div>
</template>

<script setup lang="ts">
defineProps<{
  page: object;
  html: string;
}>();
</script>
