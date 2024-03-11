<template>
  <h1>{{ $props.category.title }}</h1>
  <div v-html="$props.category.details"></div>
  <h2>Definitions</h2>
  <ul class="subgridded">
    <li v-for="item in $props.category.items" :key="item.name">
      <a :href="item.route">
        <code>{{ item.name }}</code>
      </a>
      <span>{{ item.oneliner }}</span>
    </li>
  </ul>

  <br />
  <br />
  <details><pre>{{ JSON.stringify($props, null, 2) }}</pre></details>
</template>

<script setup lang="ts">
defineProps<{
  page: object;
  category: object;
}>();
</script>
