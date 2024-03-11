<template>
  <h1>{{ $props.type.name }}</h1>
  <div v-html="$props.type.details"></div>

  <br />
  <br />
  <details><pre>{{ JSON.stringify($props, null, 2) }}</pre></details>
</template>

<script setup lang="ts">
defineProps<{
  page: object;
  type: object;
}>();
</script>
