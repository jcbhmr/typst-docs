<template>
  <h1>{{ $props.symbols.name }}</h1>
  <div v-html="$props.symbols.details"></div>
  <table>
    <tbody>
      <tr v-for="item in $props.symbols.list">
        <td>
          <b style="font-size: 1.2em"><code>{{ item.name }}</code></b>
          <br /><code>\u{{ "{" }}{{ item.codepoint.toString(16).toUpperCase() }}{{ "}" }}</code>
          <template v-if="item.markupShorthand"><br /><code>{{ item.markupShorthand }}</code> (in markup)</template>
          <template v-if="item.mathShorthand"><br /><code>{{ item.mathShorthand }}</code> (in math)</template>
        </td>
        <td><b><Badge>{{ String.fromCodePoint(item.codepoint) }}</Badge></b> {{ item.unicodeName }} <small>(U+{{ item.codepoint.toString(16).toUpperCase() }})</small></td>
      </tr>
    </tbody>
  </table>

  <br />
  <br />
  <details><pre>{{ JSON.stringify($props, null, 2) }}</pre></details>
</template>

<script setup lang="ts">
defineProps<{
  page: object;
  symbols: object;
}>();
</script>