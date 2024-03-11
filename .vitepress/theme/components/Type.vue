<template>
  <pre>{{ JSON.stringify($params, null, 2) }}</pre>
  
</template>

<script setup lang="ts">
defineProps<{
  page: object;
  type: object;
}>();
</script>
