<script setup>
import { useData } from 'vitepress'

const { params } = useData()
console.log(params.value.page)

if (params.value.page.body.kind === 'packages') {
    if (typeof window !== "undefined") {
        location.replace("https://typst.app/docs/packages/")
    }
}

let paramsCode = ""
if (params.value.page.body.kind === 'func') {
    const paramsCodeParts = params.value.page.body.content.params.map(x => {
        if (x.named) {
            return `${x.name}: ${x.types.join(" | ")}`
        } else {
            return x.types.join(" | ")
        }
    })
    if (paramsCodeParts.length === 0) {
        paramsCode = `${params.value.page.body.content.name}()`
    } else if (paramsCodeParts.length === 1) {
        paramsCode = `${params.value.page.body.content.name}(${paramsCodeParts})`
    } else {
        paramsCode = `${params.value.page.body.content.name}(${paramsCodeParts.map(x => "\n  " + x)}\n)`
    }
    paramsCode += ` -> ${params.value.page.body.content.returns.join(" | ")}`
}
</script>

<template v-if="$params.page.body.kind === 'html'">
<div v-html="$params.page.body.content"></div>
</template>

<template v-else-if="$params.page.body.kind === 'category'">
</template>

<template v-else-if="$params.page.body.kind === 'func'">

# {{ $params.page.body.content.title }}

<div v-html="$params.page.body.content.details"></div>

## Parameters

```js-vue
{{ paramsCode }}
```

</template>

<template v-else-if="$params.page.body.kind === 'group'">

</template>

<template v-else-if="$params.page.body.kind === 'type'">

</template>

<template v-else-if="$params.page.body.kind === 'symbols'">

</template>
