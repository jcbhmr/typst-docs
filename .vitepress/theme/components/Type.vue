<template>
        <h1><span :class="'pill pill-obj'">{{ $params.type.name }}</span></h1>
        <div v-html="$params.type.details"></div>

        <h2>Constructor</h2>
        <div v-html="$params.type.constructor.details"></div>
        <div v-html="$params.type.constructor.example"></div>
    
        <h2>Definitions</h2>
        <div v-for="(scope, index) in $params.type.scope" :key="index">
        <h3><span :class="'pill pill-obj'">{{ scope.name }}</span></h3>
        <div v-if="scope.details" v-html="scope.details"></div>
        </div>
</template>

<script setup lang="ts">
defineProps<{
  page: object,
  type: object,
}>()
</script>