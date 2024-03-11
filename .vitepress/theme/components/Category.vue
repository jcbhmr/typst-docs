<template>
  <h1>{{ $params.category.title }}</h1>
  <div v-html="$params.category.details"></div>
  <h2>Definitions</h2>
  <ul class="subgridded">
    <li v-for="item in $params.category.items" :key="item.name">
      <a :href="item.route">
        <code>{{ item.name }}</code>
      </a>
      <span>{{ item.oneliner }}</span>
    </li>
  </ul>
</template>

<script setup lang="ts">
defineProps<{
  page: object;
  category: object;
}>();
</script>
